use axum::{extract::Query, extract::State, response::Html};

use crate::infra::auth::{Client, Code};

pub async fn google(
    State(google_client): State<impl Client>,
    Query(code): Query<Code>,
) -> Html<String> {
    let user_info = google_client.user_info(code).await.unwrap();

    Html(format!("{:#?}", user_info))
}
