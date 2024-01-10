use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use tera::{Context, Tera};

pub(crate) async fn index(State(tera): State<Tera>) -> impl IntoResponse {
    let result = tera.render("home/index.html", &Context::default()).unwrap();
    Html(result)
}
