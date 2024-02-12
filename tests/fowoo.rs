use std::{env, fs};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use cucumber::{then, when, writer, World, WriterExt as _};
use fowoo::{controller, infra::http::Response};
use tera::Tera;

#[derive(World, Debug)]
struct FowooWorld {
    tera: Tera,
    response: Response,
}

impl Default for FowooWorld {
    fn default() -> Self {
        let template_dir_path = env::var("FOWOO_TEMPLATE_DIR_PATH")
            .unwrap_or("/workspace/templates/**/*.html".to_string());
        let tera = Tera::new(&template_dir_path).unwrap();
        let response = (StatusCode::OK, HeaderMap::new(), "".to_string());

        Self { tera, response }
    }
}

#[when(expr = "the user visits the homepage")]
async fn the_user_vists_the_homepage(world: &mut FowooWorld) {
    world.response = controller::home::index(State(world.tera.clone())).await;
}

#[when(expr = "the user vists the login page")]
async fn the_user_vists_the_login_page(world: &mut FowooWorld) {
    world.response = controller::login::index(State(world.tera.clone())).await;
}

#[then(expr = "the return status is {int}")]
async fn the_return_status_is(world: &mut FowooWorld, number: u16) {
    assert_eq!(StatusCode::from_u16(number).unwrap(), world.response.0)
}

#[tokio::main]
async fn main() {
    let file_path = env::var("CODECOV_REPORT_PATH").unwrap_or("/workspace/target".to_string());
    let file = fs::File::create(format!("{}/junit.xml", file_path)).unwrap();

    FowooWorld::cucumber()
        .with_writer(
            writer::Basic::stdout()
                .summarized()
                .tee::<FowooWorld, _>(writer::JUnit::for_tee(file, 0))
                .normalized(),
        )
        .fail_on_skipped()
        .run("tests/features/login.feature")
        .await;
}
