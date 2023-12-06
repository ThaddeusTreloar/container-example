use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use reqwest::Client;
use shared::{
    prelude::*,
    result::LoggableResult,
    state::{
        combo::{Combo, self, MaybeCombo, PartialCombo},
        entity::{self, Entity},
        property::{PartialProperty, Property, self},
    },
};

use tokio::sync::OnceCell;
use tracing::{error, info, span, Level, warn};

pub fn get_router() -> Router<Arc<Client>>
{
    Router::new()
        .route("/combo/:name", get(get_combo))
        .route("/combo/:name", delete(delete_combo))
        .route("/combo/:name", post(post_combo))
        .route("/combo/:name", patch(patch_combo))
}

async fn get_port_for_service(service: &str) -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = format!("{}_port", service.to_uppercase());

            match std::env::var(service_key) {
                Ok(port) => port,
                Err(_) => "8080".to_string(),
            }
        })
        .await
        .clone()
}

async fn get_combo(
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = generate_trace_id();

    let span = span!(Level::INFO, "get_property", id = id);
    let _enter = span.enter();

    info!("req: name={}", name);

    let entity_port = get_port_for_service("entity").await;

    let entity_address = format!("http://localhost:{entity_port}/entity/{name}");

    info!("requesting entity from: entity_address={}", entity_address);

    let entity_response = client
        .get(entity_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("entity_response={:?}", entity_response);

    let entity: entity::Entity = entity_response
        .json()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let property_port = get_port_for_service("property").await;

    let property_address = format!("http://localhost:{property_port}/property/{name}");

    info!("requesting property from: property_address={}", property_address);

    let property_response = client
        .get(property_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("property_response={:?}", property_response);

    let property = match property_response.status() {
        reqwest::StatusCode::OK => match property_response.json::<Property>().await {
            Ok(property) => property,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        _ => Property::default(),
    };

    let combo = Combo::from((entity, property));

    Ok(Json(combo))
}

async fn post_combo(
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
    Json(payload): Json<MaybeCombo>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = generate_trace_id();

    let span = span!(Level::INFO, "post_property", id = id);
    let _enter = span.enter();

    info!("req: payload={:?}", payload);

    // Build requests
    let entity_port = get_port_for_service("entity").await;

    let entity_address = format!("http://localhost:{entity_port}/entity/{name}");

    let property_port = get_port_for_service("property").await;

    let property_address = format!("http://localhost:{property_port}/property/{name}");

    // Build Bodies
    let entity_body = serde_json::to_string(&payload)
        .error()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let property = match payload.try_into() {
        Ok(property) => property,
        Err(e) => {
            warn!("Using default property");
            Property::default()
        },
    };
    
    let property_body = serde_json::to_string(&property)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Send requests

    info!("sending entity post to: entity_address={}", entity_address);

    let entity_response = client
        .post(entity_address)
        .header("logid", &id)
        .body(entity_body)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("entity_response={:?}", entity_response);

    info!("sending property post to: property_address={}", property_address);

    let property_response = client
        .post(property_address)
        .header("logid", &id)
        .body(property_body)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("property_response={:?}", property_response);

    Ok(StatusCode::CREATED.into_response())
}

async fn patch_combo(
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
    Json(payload): Json<PartialCombo>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = generate_trace_id();

    let span = span!(Level::INFO, "patch_property", id = id);
    let _enter = span.enter();

    info!("req: payload={:?}", payload);

    // Build Targets
    let entity_port = get_port_for_service("entity").await;

    let entity_address = format!("http://localhost:{entity_port}/entity/{name}");

    let property_port = get_port_for_service("property").await;

    let property_address = format!("http://localhost:{property_port}/property/{name}");

    // Get existing
    info!("requesting entity from: entity_address={}", entity_address);

    let entity_response = client
        .get(&entity_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("entity_response={:?}", entity_response);

    let existing_entity: entity::Entity = entity_response
        .json()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("requesting property from: property_address={}", property_address);

    let property_response = client
        .get(&property_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("property_response={:?}", property_response);

    let existing_property = match property_response.status() {
        reqwest::StatusCode::OK => match property_response.json::<Property>().await {
            Ok(property) => property,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        _ => Property::default(),
    };

    info!("existing_entity={:?}", existing_entity);

    // Update existing
    let existing = Combo::from((existing_entity, existing_property));

    let updated_combo = payload.merge(&existing);

    let updated_entity: Entity = updated_combo.clone().into();

    let updated_property: Property = updated_combo.clone().into();

    // Build Updates Bodies
    let entity_body = serde_json::to_string(&updated_entity)
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let property_body = serde_json::to_string(&updated_property)
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Send updates

    info!("sending entity patch to: entity_address={}", entity_address);

    let entity_response = client
        .post(entity_address)
        .header("logid", &id)
        .body(entity_body)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("entity_response={:?}", entity_response);

    info!("sending property patch to: property_address={}", property_address);

    let property_response = client
        .post(property_address)
        .header("logid", &id)
        .body(property_body)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("property_response={:?}", property_response);

    Ok(Json(updated_combo))
}

async fn delete_combo(
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = generate_trace_id();

    let span = span!(Level::INFO, "delete_property", id = id);
    let _enter = span.enter();

    info!("req: name={}", name);

    let entity_port = get_port_for_service("entity").await;

    let entity_address = format!("http://localhost:{entity_port}/entity/{name}");

    let property_port = get_port_for_service("property").await;

    let property_address = format!("http://localhost:{property_port}/property/{name}");

    info!("deleting entity from: entity_address={}", entity_address);

    let entity_response = client
        .delete(entity_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("response for entity deletion: response={:?}", entity_response);

    info!("deleting property from: property_address={}", property_address);

    let property_response = client
        .delete(property_address)
        .header("logid", &id)
        .send()
        .await
        .error()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("response for property deletion: response={:?}", property_response);

    Ok(StatusCode::NO_CONTENT.into_response())
}
