use std::{env, fs};

use axum::{extract::State, http::StatusCode};
use cucumber::{then, when, writer, World, WriterExt as _};
use fowoo::{
    controller,
    infra::{
        auth::{Client, Code, Error, UserInfo},
        http::Response,
    },
};

#[derive(World, Debug)]
struct FowooWorld {
    response: Response,
    oauth_client: MockOauthClient,
}

#[derive(Debug, Clone)]
struct MockOauthClient {
    redirect_uri: String,
}

impl Client for MockOauthClient {
    fn default() -> Self {
        Self {
            redirect_uri: String::default(),
        }
    }

    fn redirect_uri(&self) -> String {
        self.redirect_uri.clone()
    }

    async fn user_info(&self, _code: Code) -> Result<UserInfo, Error> {
        todo!()
    }
}

impl Default for FowooWorld {
    fn default() -> Self {
        let response = Response::default();
        let oauth_client = MockOauthClient {
            redirect_uri: "http://example.com/oauth/client".to_string(),
        };

        Self {
            response,
            oauth_client,
        }
    }
}

#[when(expr = "the user visits the homepage")]
async fn the_user_vists_the_homepage(world: &mut FowooWorld) {
    world.response = controller::home::index().await;
}

#[when(expr = "the user vists the login page")]
async fn the_user_vists_the_login_page(world: &mut FowooWorld) {
    world.response = controller::login::index().await;
}

#[when(expr = "the user visits the oauth provider page")]
async fn the_user_vists_the_oauth_provider_page(world: &mut FowooWorld) {
    world.response = controller::login::google(State(world.oauth_client.clone())).await;
}

#[then(expr = "the return status is {int}")]
async fn the_return_status_is(world: &mut FowooWorld, number: u16) {
    let status_code = match world.response {
        Response::Body { status_code, .. } => status_code,
        Response::Template { status_code, .. } => status_code,
    };

    assert_eq!(StatusCode::from_u16(number).unwrap(), status_code)
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
