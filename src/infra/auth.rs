pub(crate) mod google;

use serde::Deserialize;

pub(crate) trait Client {
    fn default() -> Self;
    fn redirect_uri(&self) -> String;
    async fn user_info(&self, code: Code) -> Result<UserInfo, Error>;
}

#[derive(Deserialize, Debug)]
pub(crate) struct Code {
    code: String,
}

#[derive(Debug)]
pub(crate) enum Error {
    Generic(String),
    Token {
        error: String,
        error_description: String,
    },
    Code(String),
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserInfo {
    email: String,
    picture: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Token {
    access_token: String,
    token_type: String,
    scope: String,
}
