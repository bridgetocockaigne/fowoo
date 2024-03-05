use axum::{
    http::{header, HeaderMap, HeaderName, StatusCode},
    response::IntoResponse,
};
use tera::Context;

use super::TEMPLATE;

#[derive(Debug, Default)]
pub(crate) struct ResponseBuilder {
    status_code: Option<StatusCode>,
    headers: Option<HeaderMap>,
    body: Option<String>,
    template: Option<String>,
    context: Option<Context>,
}

pub mod content_type {
    pub const TEXT_HTML: &str = "text/html; charset=utf-8";
}

#[derive(Debug)]
pub enum Response {
    Body {
        status_code: StatusCode,
        header_map: HeaderMap,
        body: String,
    },
    Template {
        status_code: StatusCode,
        header_map: HeaderMap,
        template: String,
        context: Context,
    },
}

impl Default for Response {
    fn default() -> Self {
        Response::Body {
            status_code: StatusCode::OK,
            header_map: HeaderMap::default(),
            body: String::default(),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Body {
                status_code,
                header_map,
                body,
            } => (status_code, header_map, body).into_response(),
            Response::Template {
                status_code,
                header_map,
                template,
                context,
            } => (
                status_code,
                header_map,
                TEMPLATE.render(&template, &context).unwrap(),
            )
                .into_response(),
        }
    }
}

impl ResponseBuilder {
    pub fn build(self) -> Response {
        if self.template.is_some() {
            Response::Template {
                status_code: self.status_code.unwrap_or(StatusCode::OK),
                header_map: self.headers.unwrap_or_default(),
                template: self.template.unwrap_or_default(),
                context: self.context.unwrap_or_default(),
            }
        } else {
            Response::Body {
                status_code: self.status_code.unwrap_or(StatusCode::OK),
                header_map: self.headers.unwrap_or_default(),
                body: self.body.unwrap_or_default(),
            }
        }
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

    pub fn with_template(self, template: String) -> Self {
        Self {
            template: Some(template),
            ..self
        }
    }

    pub fn with_context(self, context: Context) -> Self {
        Self {
            context: Some(context),
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
