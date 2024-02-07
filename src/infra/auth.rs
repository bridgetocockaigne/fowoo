pub mod google;

use serde::Deserialize;

pub trait Client {
    fn default() -> Self;
    fn redirect_uri(&self) -> String;
    #[allow(async_fn_in_trait)]
    async fn user_info(&self, code: Code) -> Result<UserInfo, Error>;
}

#[derive(Deserialize, Debug)]
pub struct Code {
    code: String,
}

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Token {
        error: String,
        error_description: String,
    },
    Code(String),
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    email: String,
    picture: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
}
