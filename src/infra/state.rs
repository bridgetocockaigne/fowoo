use axum::extract::FromRef;

use super::auth::{google, Client};

#[derive(Clone)]
pub struct State {
    pub(crate) google_client: google::Client,
}

impl State {
    pub fn new() -> Self {
        let google_client = google::Client::default();

        Self { google_client }
    }
}

impl FromRef<State> for google::Client {
    fn from_ref(input: &State) -> Self {
        input.google_client.clone()
    }
}
