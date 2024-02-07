use std::env;

use axum::{routing::get, Router};
use fowoo::controller::{auth, home, login};
use fowoo::infra::{auth::google::Client, state::State};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let template_dir_path =
        env::var("FOWOO_TEMPLATE_DIR_PATH").unwrap_or("/workspace/templates/**/*.html".to_string());

    let state = State::new(&template_dir_path);

    let statics_dir_path =
        env::var("FOWOO_STATICS_DIR_PATH").unwrap_or("/workspace/statics".to_string());

    let serve_dir = ServeDir::new(statics_dir_path);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home::index))
        .route("/login", get(login::index))
        .route(
            "/login/google",
            get::<_, (_, axum::extract::State<Client>), _>(login::google),
        )
        .route(
            "/auth/google",
            get::<_, (_, axum::extract::State<Client>, _), _>(auth::google),
        )
        .with_state(state)
        .nest_service("/statics", serve_dir);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
