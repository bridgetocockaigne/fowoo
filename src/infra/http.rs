use axum::http::{header, HeaderMap, HeaderName, StatusCode};

#[derive(Debug, Default)]
pub(crate) struct ResponseBuilder {
    status_code: Option<StatusCode>,
    headers: Option<HeaderMap>,
    body: Option<String>,
}

pub mod content_type {
    pub const TEXT_HTML: &str = "text/html; charset=utf-8";
}

pub type Response = (StatusCode, HeaderMap, String);

impl ResponseBuilder {
    pub fn build(self) -> Response {
        (
            self.status_code.unwrap_or(StatusCode::OK),
            self.headers.unwrap_or_default(),
            self.body.unwrap_or_default(),
        )
    }

    pub fn with_status_code(self, status_code: StatusCode) -> Self {
        Self {
            status_code: Some(status_code),
            ..self
        }
    }

    pub fn with_content_type(self, content_type_value: &str) -> Self {
        self.with_header(header::CONTENT_TYPE, content_type_value.to_string())
    }

    pub fn with_header(self, header_name: HeaderName, header_value: String) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(header_name, header_value.parse().unwrap());

        Self {
            headers: Some(headers),
            ..self
        }
    }

    pub fn with_body(self, body: String) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }
}
