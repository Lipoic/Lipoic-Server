use urlencoding::encode;
use serde::Deserialize;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub struct GoogleOAuth<'a> {
    client_secret: String,
    client_id: String,
    issuer: String,
    redirect_path: &'a str
}

#[derive(Deserialize, Debug)]
pub struct GoogleOauthAccessTokenInfo {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
    id_token: String,
}

impl GoogleOAuth<'_> {
    pub fn new(client_secret: String, client_id: String, issuer: String, redirect_path: &str) -> GoogleOAuth {
        GoogleOAuth {
            client_secret,
            client_id,
            issuer,
            redirect_path
        }
    }

    /// get google oauth url
    pub fn get_auth_url(&self) -> String {
        let scope = encode("https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email");
        let redirect_uri = format!("{}{}", self.issuer, self.redirect_path);
        format!(
            "{}?client_id={}&response_type=code&scope={}&redirect_uri={}",
            GOOGLE_AUTH_URL,
            self.client_id,
            scope,
            encode(redirect_uri.as_ref())
        )
    }

    /// authorization code
    /// return OAuth access token info
    pub async fn authorization_code(&self, code: String, path: &str) -> Result<GoogleOauthAccessTokenInfo, Box<dyn std::error::Error>> {
        let form_data = [
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", "authorization_code".to_string()),
            ("code", code),
            ("redirect_uri", format!("{}{}", self.issuer, path))
        ];

        let response = reqwest::Client::new()
            .post(GOOGLE_TOKEN_URL)
            .form(&form_data)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        Ok(response.json::<GoogleOauthAccessTokenInfo>().await?)
    }
}