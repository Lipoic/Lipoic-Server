use reqwest::Error;
use serde::Deserialize;
use urlencoding::encode;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USER_INFO: &str = "https://www.googleapis.com/oauth2/v1/userinfo?alt=json";

pub struct GoogleOAuth<'a> {
    client_secret: String,
    client_id: String,
    issuer: String,
    redirect_path: &'a str,
}

#[derive(Deserialize)]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub expires_in: i32,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct GoogleAccountInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

impl GoogleOAuth<'_> {
    pub fn new(
        client_secret: String,
        client_id: String,
        issuer: String,
        redirect_path: &str,
    ) -> GoogleOAuth {
        GoogleOAuth {
            client_secret,
            client_id,
            issuer,
            redirect_path,
        }
    }

    /// get google oauth url
    ///
    /// return one url [`String`]
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
    ///
    /// return [`AccessTokenInfo`]
    pub async fn authorization_code(&self, code: String) -> Result<AccessTokenInfo, Error> {
        let form_data = [
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", "authorization_code".to_string()),
            ("code", code),
            (
                "redirect_uri",
                format!("{}{}", self.issuer, self.redirect_path),
            ),
        ];

        let response = reqwest::Client::new()
            .post(GOOGLE_TOKEN_URL)
            .form(&form_data)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        response.json::<AccessTokenInfo>().await
    }
}

impl AccessTokenInfo {
    /// request user info
    ///
    /// return [`GoogleAccountInfo`]
    pub async fn get_user_info(&self) -> Result<GoogleAccountInfo, Box<dyn std::error::Error>> {
        let response = reqwest::Client::new()
            .get(GOOGLE_USER_INFO)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        Ok(response.json::<GoogleAccountInfo>().await?)
    }
}
