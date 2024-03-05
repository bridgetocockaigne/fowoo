use crate::infra::http::{content_type::TEXT_HTML, Response, ResponseBuilder};

pub async fn index() -> Response {
    ResponseBuilder::default()
        .with_template("home/index.html".to_string())
        .with_content_type(TEXT_HTML)
        .build()
}
