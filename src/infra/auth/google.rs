use std::env;

#[derive(Clone)]
pub struct Client {
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) response_type: String,
    pub(crate) scope: String,
    pub(crate) access_type: String,
    pub(crate) client_secret: String,
    pub(crate) grant_type: String,
}

impl super::Client for Client {
    fn default() -> Self {
        let client_id = env::var("FOWOO_GOOGLE_CLIENT_ID").unwrap();
        let redirect_uri = env::var("FOWOO_GOOGLE_REDIRECT_URI").unwrap();
        let scope = env::var("FOWOO_GOOGLE_SCOPE").unwrap();
        let client_secret = env::var("FOWOO_GOOGLE_SECRET").unwrap();

        let access_type = "offline".to_string();
        let response_type = "code".to_string();
        let grant_type = "authorization_code".to_string();

        Self {
            client_id,
            redirect_uri,
            response_type,
            scope,
            access_type,
            client_secret,
            grant_type,
        }
    }

    fn redirect_uri(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?scope={}&access_type={}&response_type={}&redirect_uri={}&client_id={}",
            self.scope, self.access_type, self.response_type, self.redirect_uri, self.client_id
        )
    }

    async fn user_info(&self, code: super::Code) -> Result<super::UserInfo, super::Error> {
        let params = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("code", &code.code),
            ("grant_type", &self.grant_type),
            ("redirect_uri", &self.redirect_uri),
        ];

        println!("{:#?}", params);

        let client = reqwest::Client::new();
        let token: super::Token = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let user_info: super::UserInfo = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        Ok(user_info)
    }
}
