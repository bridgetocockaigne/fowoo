use axum::extract::State;
use tera::{Context, Tera};

use crate::infra::http::{content_type::TEXT_HTML, Response, ResponseBuilder};

pub async fn index(State(tera): State<Tera>) -> Response {
    let result = tera.render("home/index.html", &Context::default()).unwrap();

    ResponseBuilder::default()
        .with_body(result)
        .with_content_type(TEXT_HTML)
        .build()
}
