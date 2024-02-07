use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use cucumber::{given, then, when, World};
use fowoo::{
    controller,
    infra::auth::{Client, Code, Error, UserInfo},
};

#[derive(Debug, Clone)]
struct MockClient {}

impl Client for MockClient {
    fn default() -> Self {
        Self {}
    }

    fn redirect_uri(&self) -> String {
        String::new()
    }

    async fn user_info(&self, _code: Code) -> Result<UserInfo, Error> {
        let user_info = UserInfo {
            email: "email@example.com".to_string(),
            picture: "pic.jpg".to_string(),
        };

        Ok(user_info)
    }
}

#[derive(Debug, World)]
struct FowooWorld {
    code: Code,
    client: MockClient,
    result: Html<String>,
}

impl Default for FowooWorld {
    fn default() -> Self {
        Self {
            code: Code::default(),
            client: MockClient::default(),
            result: Html(String::new()),
        }
    }
}

#[given(expr = "a code '{word}'")]
fn a_code(world: &mut FowooWorld, code: String) {
    world.code = Code { code }
}

#[given(expr = "a mock client provider")]
fn a_mock_client(world: &mut FowooWorld) {
    world.client = MockClient::default();
}

#[when(expr = "I call the provider with the code")]
async fn call_provider_with_code(world: &mut FowooWorld) {
    world.result =
        controller::auth::google(State(world.client.clone()), Query(world.code.clone())).await;
}

#[then(expr = "I get an OK response")]
fn get_an_ok_response(world: &mut FowooWorld) {
    let status = world.result.clone().into_response().status();
    assert_eq!(StatusCode::OK, status);
}

#[tokio::main]
async fn main() {
    FowooWorld::cucumber()
        .fail_on_skipped()
        .run("tests/feature/auth.feature")
        .await;
}
