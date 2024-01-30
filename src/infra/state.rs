use axum::extract::FromRef;
use tera::Tera;

use super::auth::{google, Client};

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) tera: Tera,
    pub(crate) google_client: google::Client,
}

impl State {
    pub(crate) fn new(template_dir_path: &str) -> Self {
        let tera = Tera::new(template_dir_path).unwrap();
        let google_client = google::Client::default();

        Self {
            tera,
            google_client,
        }
    }
}

impl FromRef<State> for google::Client {
    fn from_ref(input: &State) -> Self {
        input.google_client.clone()
    }
}

impl FromRef<State> for Tera {
    fn from_ref(input: &State) -> Self {
        input.tera.clone()
    }
}
