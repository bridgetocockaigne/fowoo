use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};
use tera::{Context, Tera};

use crate::infra::auth::Client;

pub(crate) async fn index(State(tera): State<Tera>) -> impl IntoResponse {
    let result = tera
        .render("login/index.html", &Context::default())
        .unwrap();
    Html(result)
}

pub(crate) async fn google(State(google_client): State<impl Client>) -> impl IntoResponse {
    let redirect_uri = google_client.redirect_uri();

    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, redirect_uri.parse().unwrap());

    (StatusCode::TEMPORARY_REDIRECT, headers)
}

#[cfg(test)]
mod tests {
    use std::env;

    use axum::{extract::State, http::StatusCode, response::IntoResponse};
    use tera::Tera;

    #[tokio::test]
    async fn index() {
        let template_dir_path = env::var("FOWOO_TEMPLATE_DIR_PATH")
            .unwrap_or("/workspace/templates/**/*.html".to_string());

        let tera = Tera::new(&template_dir_path).unwrap();
        let result = super::index(State(tera)).await.into_response();

        assert_eq!(StatusCode::OK, result.status());
    }
}
