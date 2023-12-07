use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use hyper::HeaderMap;
use reqwest::Client;
use shared::{
    prelude::*,
    result::LoggableResult,
    state::{
        combo::{self, Combo, MaybeCombo, PartialCombo},
        entity::{self, Entity},
        property::{self, PartialProperty, Property},
    }, header_helper,
};

use tokio::sync::OnceCell;
use tracing::{error, info, warn};

pub fn get_router() -> Router<Arc<Client>> {
    Router::new()
        .route("/combo/:name", get(get_combo))
        .route("/combo/:name", delete(delete_combo))
        .route("/combo/:name", post(post_combo))
        .route("/combo/:name", patch(patch_combo))
}

async fn get_port_for_service(service: &str) -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = format!("{}_port", service).to_uppercase();

            info!("got service_key={service_key}");

            match std::env::var(service_key) {
                Ok(port) => {
                    info!("got service={service}, port={port}");
                    port
                }
                Err(_) => {
                    warn!("using default port for service={service}");
                    "8080".to_string()
                }
            }
        })
        .await
        .clone()
}

async fn get_canonical_name_for_service(service: &str) -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = format!("{}_address", service).to_uppercase();

            info!("got service_key={service_key}");

            match std::env::var(service_key) {
                Ok(address) => {
                    info!("got service={service}, address={address}");
                    address
                }
                Err(_) => {
                    warn!("using default address for service={service}");
                    service.to_string()
                }
            }
        })
        .await
        .clone()
}

async fn get_address_for_servive(service: &str) -> String {
    let port = get_port_for_service(service).await;
    let domain = get_canonical_name_for_service(service).await;

    format!("http://{domain}:{port}", domain = domain, port = port)
}

async fn get_path_for_service(service: &str, path: &str) -> String {
    let address = get_address_for_servive(service).await;

    format!("{address}/{path}")
}

async fn get_combo(
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
    headers: HeaderMap
) -> Result<impl IntoResponse, StatusCode> {
    let id: String = shared::header_helper::get_logid(headers).await;

    info!("req: name={}", name);

    let entity_address = get_path_for_service("entity", format!("entity/{name}").as_str()).await;

    let property_address =
        get_path_for_service("property", format!("property/{name}").as_str()).await;

    info!("requesting entity from: entity_address={}", entity_address);

    let entity_response = match client.get(entity_address).header("logid", &id).send().await {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("entity_response={:?}", entity_response);

    let entity: entity::Entity = match entity_response.status() {
        reqwest::StatusCode::NOT_FOUND => return Err(StatusCode::NOT_FOUND),
        reqwest::StatusCode::OK => match entity_response.json().await {
            Ok(entity) => entity,
            Err(e) => {
                error!("error parsing entity: error={:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        s => {
            error!("unexpected status code: status_code={:?}", s);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!(
        "requesting property from: property_address={}",
        property_address
    );

    let property_response = match client
        .get(property_address)
        .header("logid", &id)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("property_response={:?}", property_response);

    let property = match property_response.status(){
        reqwest::StatusCode::NOT_FOUND => return Err(StatusCode::NOT_FOUND),
        reqwest::StatusCode::OK => match property_response.json().await {
            Ok(entity) => entity,
            Err(e) => {
                error!("error parsing property: error={:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        s => {
            error!("unexpected status code: status_code={:?}", s);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let combo = Combo::from((entity, property));

    Ok(Json(combo))
}

async fn post_combo(
    headers: HeaderMap,
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
    Json(payload): Json<MaybeCombo>,
) -> Result<impl IntoResponse, StatusCode> {
    let id: String = shared::header_helper::get_logid(headers).await;

    info!("req: payload={:?}", payload);

    // Build request

    let entity_address = get_path_for_service("entity", format!("entity/{name}").as_str()).await;

    let property_address =
        get_path_for_service("property", format!("property/{name}").as_str()).await;

    // Build Bodies
    let entity_body = match serde_json::to_string(&payload) {
        Ok(body) => body,
        Err(e) => {
            error!("error serializing entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let property = match payload.try_into() {
        Ok(property) => property,
        Err(e) => {
            warn!("Using default property");
            Property::default()
        }
    };

    let property_body = match serde_json::to_string(&property) {
        Ok(body) => body,
        Err(e) => {
            error!("error serializing property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Send requests

    info!("sending entity post to: entity_address={}", entity_address);

    let entity_response = match client
        .post(entity_address)
        .header("logid", &id)
        .header("content-type", "application/json")
        .body(entity_body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error sending entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !entity_response.status().is_success() {
        error!("unexpected status code: status_code={:?}, reason={:?}", entity_response.status(), entity_response.status().canonical_reason());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let property_response = match client
        .post(&property_address)
        .header("logid", &id)
        .header("content-type", "application/json")
        .body(property_body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error sending property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("entity_response={:?}", entity_response);

    info!(
        "sending property post to: property_address={}",
        property_address
    );

    info!("property_response={:?}", property_response);

    if !property_response.status().is_success() {
        error!("unexpected status code: status_code={:?}, reason={:?}", property_response.status(), property_response.status().canonical_reason());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED.into_response())
}

async fn patch_combo(
    headers: HeaderMap,    
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
    Json(payload): Json<PartialCombo>,
) -> Result<impl IntoResponse, StatusCode> {
    let id: String = shared::header_helper::get_logid(headers).await;

    info!("req: payload={:?}", payload);

    // Build Target

    let entity_address = get_path_for_service("entity", format!("entity/{name}").as_str()).await;

    let property_address =
        get_path_for_service("property", format!("property/{name}").as_str()).await;

    // Get existing
    info!("requesting entity from: entity_address={}", entity_address);

    let entity_response = match client
        .get(&entity_address)
        .header("logid", &id)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("entity_response={:?}", entity_response);

    let existing_entity: entity::Entity = match entity_response.json().await {
        Ok(entity) => entity,
        Err(e) => {
            error!("error parsing entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!(
        "requesting property from: property_address={}",
        property_address
    );

    let property_response = match client
        .get(&property_address)
        .header("logid", &id)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

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
    let entity_body = match serde_json::to_string(&updated_entity) {
        Ok(body) => body,
        Err(e) => {
            error!("error serializing entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let property_body = match serde_json::to_string(&updated_property) {
        Ok(body) => body,
        Err(e) => {
            error!("error serializing property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Send updates

    info!("sending entity patch to: entity_address={}", entity_address);

    let entity_response = match client
        .post(entity_address)
        .header("logid", &id)
        .header("content-type", "application/json")
        .body(entity_body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error sending entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("entity_response={:?}", entity_response);

    info!(
        "sending property patch to: property_address={}",
        property_address
    );

    let property_response = match client
        .post(property_address)
        .header("logid", &id)
        .header("content-type", "application/json")
        .body(property_body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error sending property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("property_response={:?}", property_response);

    Ok(Json(updated_combo))
}

async fn delete_combo(
    headers: HeaderMap,
    State(client): State<Arc<Client>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let id: String = shared::header_helper::get_logid(headers).await;

    info!("req: name={}", name);

    let entity_address = get_path_for_service("entity", format!("entity/{name}").as_str()).await;

    let property_address =
        get_path_for_service("property", format!("property/{name}").as_str()).await;

    info!("deleting entity from: entity_address={}", entity_address);

    let entity_response = match client
        .delete(entity_address)
        .header("logid", &id)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting entity: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!(
        "response for entity deletion: response={:?}",
        entity_response
    );

    info!(
        "deleting property from: property_address={}",
        property_address
    );

    let property_response = match client
        .delete(property_address)
        .header("logid", &id)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("error requesting property: error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!(
        "response for property deletion: response={:?}",
        property_response
    );

    Ok(StatusCode::NO_CONTENT.into_response())
}
