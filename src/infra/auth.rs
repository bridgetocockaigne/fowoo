pub mod google;

use serde::Deserialize;

pub trait Client {
    fn default() -> Self;
    fn redirect_uri(&self) -> String;
    fn user_info(
        &self,
        code: Code,
    ) -> impl std::future::Future<Output = Result<UserInfo, Error>> + Send;
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
