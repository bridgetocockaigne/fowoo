use axum::{extract::Query, extract::State};

use crate::infra::{
    auth::{Client, Code},
    http::{Response, ResponseBuilder},
};

pub async fn google(
    State(google_client): State<impl Client>,
    Query(code): Query<Code>,
) -> Response {
    let user_info = google_client.user_info(code).await.unwrap();

    ResponseBuilder::default()
        .with_body(format!("{:#?}", user_info))
        .build()
}
