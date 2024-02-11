use axum::{
    extract::State,
    http::{header, StatusCode},
};
use tera::{Context, Tera};

use crate::infra::{
    auth::Client,
    http::{content_type::TEXT_HTML, Response, ResponseBuilder},
};

pub async fn index(State(tera): State<Tera>) -> Response {
    let result = tera
        .render("login/index.html", &Context::default())
        .unwrap();

    ResponseBuilder::default()
        .with_body(result)
        .with_content_type(TEXT_HTML)
        .build()
}

pub async fn google(State(google_client): State<impl Client>) -> Response {
    let redirect_uri = google_client.redirect_uri();

    ResponseBuilder::default()
        .with_header(header::LOCATION, redirect_uri)
        .with_status_code(StatusCode::TEMPORARY_REDIRECT)
        .build()
}
