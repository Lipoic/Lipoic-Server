use reqwest::Error;
use serde::Deserialize;
use urlencoding::encode;

use crate::util::get_redirect_uri_by_path;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USER_INFO: &str = "https://www.googleapis.com/oauth2/v1/userinfo?alt=json";

const FACEBOOK_AUTH_URL: &str = "https://www.facebook.com/dialog/oauth";
const FACEBOOK_TOKEN_URL: &str = "https://graph.facebook.com/v14.0/oauth/access_token";
const FACEBOOK_USER_INFO: &str = "https://graph.facebook.com/v14.0";

pub enum OauthAccountType {
    Google,
    Facebook,
}

pub struct OAuthData<'a> {
    account_type: OauthAccountType,
    client_secret: String,
    client_id: String,
    issuer: String,
    redirect_path: &'a str,
}

#[derive(Deserialize)]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub expires_in: i32,
    #[serde(skip_deserializing)]
    pub scope: String,
    pub token_type: String,
    #[serde(skip_deserializing)]
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

#[derive(Deserialize)]
pub struct FacebookAccountInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: String,
    pub picture: FacebookAccountPicture,
}

#[derive(Deserialize)]
pub struct FacebookAccountPicture {
    pub data: FacebookAccountPictureData,
}

#[derive(Deserialize)]
pub struct FacebookAccountPictureData {
    pub height: i32,
    pub is_silhouette: bool,
    pub url: String,
    pub width: i32,
}

impl OAuthData<'_> {
    pub fn new(
        account_type: OauthAccountType,
        client_secret: String,
        client_id: String,
        issuer: String,
        redirect_path: &str,
    ) -> OAuthData {
        OAuthData {
            account_type,
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
        let scope = match self.account_type {
            OauthAccountType::Google => 
                 encode("https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email"),
            OauthAccountType::Facebook => 
                 encode("public_profile,email"),
        };
        
        let auth_url = match self.account_type {
            OauthAccountType::Google => GOOGLE_AUTH_URL,
            OauthAccountType::Facebook => FACEBOOK_AUTH_URL,
        };

        let redirect_uri = get_redirect_uri_by_path(&self.issuer, self.redirect_path);

        format!(
            "{}?client_id={}&response_type=code&scope={}&redirect_uri={}",
            auth_url,
            self.client_id,
            scope,
            encode(redirect_uri.as_ref())
        )
    }

    /// authorization code
    ///
    /// return [`AccessTokenInfo`]
    pub async fn authorization_code(&self, code: String) -> Result<AccessTokenInfo, Error> {
        let mut form_data = vec![
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", "authorization_code".to_string()),
            ("code", code),
            (
                "redirect_uri",
                get_redirect_uri_by_path(&self.issuer, self.redirect_path),
            ),
        ];

        if matches!(self.account_type, OauthAccountType::Google) {
            form_data.push(("grant_type", "authorization_code".to_string()));
        }

        let token_url = match self.account_type {
            OauthAccountType::Google => GOOGLE_TOKEN_URL,
            OauthAccountType::Facebook => FACEBOOK_TOKEN_URL,
        };

        let response = reqwest::Client::new()
            .post(token_url)
            .form(&form_data)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        response.json::<AccessTokenInfo>().await
    }
}

impl AccessTokenInfo {
    /// request google user info
    ///
    /// return [`GoogleAccountInfo`]
    pub async fn get_google_user_info(
        &self,
    ) -> Result<GoogleAccountInfo, Box<dyn std::error::Error>> {
        let response = reqwest::Client::new()
            .get(GOOGLE_USER_INFO)
            .bearer_auth(self.access_token.clone())
            .send()
            .await?;

        Ok(response.json::<GoogleAccountInfo>().await?)
    }

    /// request facebook user info
    ///
    /// return [`FacebookAccountInfo`]
    pub async fn get_facebook_user_info(
        &self,
    ) -> Result<FacebookAccountInfo, Box<dyn std::error::Error>> {
        let response = reqwest::Client::new()
            .get(format!("{}/me?access_token={}", FACEBOOK_USER_INFO,self.access_token.clone()))
            .send()
            .await?;

        Ok(response.json::<FacebookAccountInfo>().await?)
    }
}
