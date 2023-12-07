use core::panic;
use std::{sync::Arc, time::Duration};

use anyhow::Error;
use axum::{routing::{get, post, patch, delete}, Router, response::{IntoResponse, Response}, http::{StatusCode, HeaderValue}, extract::{Request, State}, Json, body::{Body, to_bytes}};
use hyper::header;
use reqwest::Client;
use shared::{init::init_tracing, trace::generate_trace_id, header_helper::get_logid};
use tokio::sync::OnceCell;
use tracing::{info, error, warn};

//mod biz_router;

fn convert_status(req_status: reqwest::StatusCode) -> StatusCode  {
    match req_status {
        reqwest::StatusCode::OK => StatusCode::OK,
        reqwest::StatusCode::CREATED => StatusCode::CREATED,
        reqwest::StatusCode::ACCEPTED => StatusCode::ACCEPTED,
        reqwest::StatusCode::NO_CONTENT => StatusCode::NO_CONTENT,
        reqwest::StatusCode::BAD_REQUEST => StatusCode::BAD_REQUEST,
        reqwest::StatusCode::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
        reqwest::StatusCode::FORBIDDEN => StatusCode::FORBIDDEN,
        reqwest::StatusCode::NOT_FOUND => StatusCode::NOT_FOUND,
        reqwest::StatusCode::METHOD_NOT_ALLOWED => StatusCode::METHOD_NOT_ALLOWED,
        reqwest::StatusCode::CONFLICT => StatusCode::CONFLICT,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
        reqwest::StatusCode::SERVICE_UNAVAILABLE => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR
    }

}

async fn get_service_name() -> String {
    OnceCell::<String>::new()
        .get_or_init(|| async {
            let service_key = "SERVICE_NAME";

            info!("getting key={service_key}");

            match std::env::var(service_key) {
                Ok(service) => {
                    info!("got service={service}");
                    service
                }
                Err(_) => {
                    panic!("{} not set", service_key);
                }
            }
        })
        .await
        .clone()
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

async fn get_backing_addres() -> String {
    let service = get_service_name().await;

    let port = get_port_for_service("service").await;

    let address = get_canonical_name_for_service("service").await;

    format!("http://{}:{}", address, port)
}

async fn rewrite_uri(req: &mut Request<Body>) {
    let backing_address = get_backing_addres().await;

    let uri = req.uri_mut();

    let path = uri.path_and_query().unwrap().as_str();

    let new_path = format!("{}{}", backing_address, path);

    *uri = new_path.parse().unwrap();
}

async fn handle_get(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let headers = req.headers().clone();

    let id = get_logid(headers).await;

    let span = tracing::info_span!("proxy_handler", trace_id = id.as_str());

    let _guard = span.enter();
    
    info!("Rewriting uri");
    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");

    let res = state
        .get(req_uri)
        .header("logid", id)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}


async fn handle_delete(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let headers = req.headers().clone();

    let id = get_logid(headers).await;

    let span = tracing::info_span!("proxy_handler", trace_id = id.as_str());

    let _guard = span.enter();

    info!("Rewriting uri");
    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");

    let res = state
        .delete(req_uri)
        .header("logid", id)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        info!("Got response: {res:?}");

        let headers = res.headers().clone();
    
        let status = res.status();
    
        let mut response = match res.text().await {
            Ok(body) => Response::new(Body::from(body)),
            Err(_) => Response::new(Body::from(""))
        };
    
        let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
            Some(content_type) => content_type.as_bytes(),
            None => b""
        };
    
        response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());
    
        *response.status_mut() = convert_status(status);
    
        info!("{response:?}");

    Ok(response)
}

async fn handle_post(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let headers = req.headers().clone();

    let id = get_logid(headers).await;

    let span = tracing::info_span!("proxy_handler", trace_id = id.as_str());

    let _guard = span.enter();

    info!("Rewriting uri");

    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");
    
    let req_headers = req.headers().clone();
    let req_content_type = req_headers.get(header::CONTENT_TYPE).unwrap().as_bytes();
    info!("Got content type: {req_content_type:?}");
    let body = match to_bytes(req.into_body(), 1024).await {
        Ok(body) => body,
        Err(e) => {
            error!("Error reading body: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let res = state
        .post(req_uri)
        .body(body)
        .header("logid", id)
        .header(reqwest::header::CONTENT_TYPE, req_content_type)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}

async fn handle_patch(State(state): State<Arc<Client>>, mut req: Request) -> Result<Response, StatusCode> {
    let headers = req.headers().clone();

    let id = get_logid(headers).await;

    let span = tracing::info_span!("proxy_handler", trace_id = id.as_str());

    let _guard = span.enter();
    
    info!("Rewriting uri");

    rewrite_uri(&mut req).await;
    
    let req_uri = req.uri().to_string();
    info!("Sending request: {req_uri}");
    
    let req_headers = req.headers().clone();
    let req_content_type = req_headers.get(header::CONTENT_TYPE).unwrap().as_bytes();
    info!("Got content type: {req_content_type:?}");
    let body = match to_bytes(req.into_body(), 1024).await {
        Ok(body) => body,
        Err(e) => {
            error!("Error reading body: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let res = state
        .patch(req_uri)
        .body(body)
        .header("logid", id)
        .header(reqwest::header::CONTENT_TYPE, req_content_type)
        .send()
        .await
        .map_err(|e| {
            error!("Error sending request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Got response: {res:?}");

    let headers = res.headers().clone();

    let status = res.status();

    let mut response = match res.text().await {
        Ok(body) => Response::new(Body::from(body)),
        Err(_) => Response::new(Body::from(""))
    };

    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => content_type.as_bytes(),
        None => b""
    };

    response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_bytes(content_type).unwrap());

    *response.status_mut() = convert_status(status);

    info!("{response:?}");

    Ok(response)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Error> {
    init_tracing();

    info!("Starting proxy_handler");

    info!("Creating client pool");

    let app_state = Arc::new(
        Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?
    );

    info!("Creating routers");

    let router = Router::new()
        .route("/*path", get(handle_get))
        .route("/*path", post(handle_post))
        .route("/*path", patch(handle_patch))
        .route("/*path", delete(handle_delete))
        .with_state(app_state.clone());

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    info!("Creating listener");
    let listener = tokio::net::TcpListener::
        bind(format!("0.0.0.0:{port}")).await?;

    info!("Starting server");
    Ok(
        axum::serve(
            listener, 
            router
        ).await?
    )
}