pub mod google;

use serde::Deserialize;

pub trait Client {
    fn default() -> Self;
    fn redirect_uri(&self) -> String;
    #[allow(async_fn_in_trait)]
    async fn user_info(&self, code: Code) -> Result<UserInfo, Error>;
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Code {
    pub code: String,
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
    pub email: String,
    pub picture: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}
