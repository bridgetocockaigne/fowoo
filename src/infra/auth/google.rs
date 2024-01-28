use std::env;

use serde::Deserialize;

#[derive(Clone)]
pub(crate) struct Client {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    access_type: String,
    client_secret: String,
}

#[derive(Deserialize)]
pub(crate) struct Code {
    pub(crate) code: String,
}

impl Client {
    pub(crate) fn redirect_uri(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?scope={}&access_type={}&response_type={}&redirect_uri={}&client_id={}",
            self.scope, self.access_type, self.response_type, self.redirect_uri, self.client_id
        )
    }
}

impl Default for Client {
    fn default() -> Self {
        let client_id = env::var("FOWOO_GOOGLE_CLIENT_ID").unwrap();
        let redirect_uri = env::var("FOWOO_GOOGLE_REDIRECT_URI").unwrap();
        let scope = env::var("FOWOO_GOOGLE_SCOPE").unwrap();
        let client_secret = env::var("FOWOO_GOOGLE_SECRET").unwrap();

        let access_type = "offline".to_string();
        let response_type = "code".to_string();

        Self {
            client_id,
            redirect_uri,
            response_type,
            scope,
            access_type,
            client_secret,
        }
    }
}
