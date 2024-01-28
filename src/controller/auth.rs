use axum::{extract::Query, extract::State, response::IntoResponse};

use crate::infra::auth::google;

pub(crate) async fn google(
    State(google_client): State<google::Client>,
    Query(code): Query<google::Code>,
) -> impl IntoResponse {
}
