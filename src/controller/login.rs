use axum::{
    extract::State,
    http::{header, StatusCode},
};

use crate::infra::{
    auth::Client,
    http::{content_type::TEXT_HTML, Response, ResponseBuilder},
};

pub async fn index() -> Response {
    ResponseBuilder::default()
        .with_template("login/index.html".to_string())
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
