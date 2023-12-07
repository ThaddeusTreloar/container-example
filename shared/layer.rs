use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Request}, http::{HeaderName, HeaderValue},
};
use tower_http::{classify::{ServerErrorsAsFailures, SharedClassifier}, set_header::SetRequestHeaderLayer};
use tracing::Span;

use crate::{header_helper::{get_logid_blocking, LOGID_HEADER}, prelude::generate_trace_id};

type TraceLayer = tower_http::trace::TraceLayer<SharedClassifier<ServerErrorsAsFailures>, fn(&hyper::Request<Body>) -> Span>;
type LogidLayer<T> = tower_http::set_header::SetRequestHeaderLayer<for<'a> fn(&'a T) -> Option<HeaderValue>>;

pub fn tracing_layer() -> TraceLayer {
    tower_http::trace::TraceLayer::new_for_http()
    .make_span_with(trace_layer_inner)
}

pub fn logid_layer<T>() -> LogidLayer<T> {
    SetRequestHeaderLayer::if_not_present(HeaderName::from_static(LOGID_HEADER), generate_trace_id_for_layer)
}

fn trace_layer_inner(request: &Request) -> Span {
    let caller = match request.extensions().get::<ConnectInfo<SocketAddr>>() {
        Some(addr) => addr.to_string(),
        None => "unknown".to_string(),
    };

    let logid = get_logid_blocking(request.headers());

    tracing::info_span!(
        "request",
        logid = %logid,
        method = %request.method(),
        uri = %request.uri(),
        caller
    )
}


pub fn generate_trace_id_for_layer<T>(_: &T) -> Option<HeaderValue> {
    generate_trace_id().parse().ok()
}
