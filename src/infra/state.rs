use axum::extract::FromRef;
use tera::Tera;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) tera: Tera,
}

impl State {
    pub(crate) fn new(template_dir_path: &str) -> Self {
        let tera = Tera::new(template_dir_path).unwrap();

        Self { tera }
    }
}

impl FromRef<State> for Tera {
    fn from_ref(input: &State) -> Self {
        input.tera.clone()
    }
}
