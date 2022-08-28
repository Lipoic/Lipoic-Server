use database::model::auth::user::ConnectType;
use reqwest::Error;
use serde::Deserialize;
use urlencoding::encode;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USER_INFO: &str = "https://www.googleapis.com/oauth2/v1/userinfo?alt=json";

const FACEBOOK_AUTH_URL: &str = "https://www.facebook.com/dialog/oauth";
const FACEBOOK_TOKEN_URL: &str = "https://graph.facebook.com/v14.0/oauth/access_token";
const FACEBOOK_USER_INFO: &str = "https://graph.facebook.com/v14.0";

pub struct OAuthData {
    pub account_type: ConnectType,
    pub client_secret: String,
    pub client_id: String,
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub expires_in: i32,
    /// Appears only in google OAuth
    #[serde(skip_deserializing)]
    pub scope: String,
    pub token_type: String,
    /// Appears only in google OAuth
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

#[derive(Deserialize)]
pub struct OAuthAccountInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub verified_email: bool,
}

impl OAuthAccountInfo {
    fn from_google(google_account_info: GoogleAccountInfo) -> Self {
        OAuthAccountInfo {
            id: google_account_info.id,
            name: google_account_info.name,
            email: google_account_info.email,
            picture: google_account_info.picture,
            verified_email: google_account_info.verified_email,
        }
    }

    fn from_facebook(facebook_account_info: FacebookAccountInfo) -> Self {
        OAuthAccountInfo {
            id: facebook_account_info.id,
            name: facebook_account_info.name,
            email: facebook_account_info.email,
            picture: facebook_account_info.picture.data.url,
            verified_email: true,
        }
    }
}

impl OAuthData {
    /// get google oauth url
    ///
    /// return one url [`String`]
    pub fn get_auth_url(&self) -> String {
        let scope = match self.account_type {
            ConnectType::Google => "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email",
            ConnectType::Facebook => "public_profile,email",
        };

        let auth_url = match self.account_type {
            ConnectType::Google => GOOGLE_AUTH_URL,
            ConnectType::Facebook => FACEBOOK_AUTH_URL,
        };

        format!(
            "{}?client_id={}&response_type=code&scope={}&redirect_uri={}",
            auth_url,
            self.client_id,
            encode(scope),
            encode(&self.redirect_uri)
        )
    }

    /// get access token info by code
    ///
    /// return [`AccessTokenInfo`]
    pub async fn authorization_code(&self, code: String) -> Result<AccessTokenInfo, Error> {
        let mut redirect_uri = self.redirect_uri.to_string();

        // Because Facebook requires "/" at the end of the redirect uri
        if matches!(self.account_type, ConnectType::Facebook) && !redirect_uri.ends_with('/') {
            redirect_uri = format!("{}/", redirect_uri);
        }

        let mut form_data = vec![
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ];

        if matches!(self.account_type, ConnectType::Google) {
            form_data.push(("grant_type", "authorization_code".to_string()));
        }

        let token_url = match self.account_type {
            ConnectType::Google => GOOGLE_TOKEN_URL,
            ConnectType::Facebook => FACEBOOK_TOKEN_URL,
        };

        let client = reqwest::Client::new();

        let builder = match self.account_type {
            ConnectType::Google => client.post(token_url).form(&form_data),
            ConnectType::Facebook => client.get(token_url).query(&form_data),
        };

        let response = builder.send().await?;

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
            .get(format!(
                "{}/me?fields=id,first_name,last_name,name,email,picture&access_token={}",
                FACEBOOK_USER_INFO,
                self.access_token.clone()
            ))
            .send()
            .await?;

        Ok(response.json::<FacebookAccountInfo>().await?)
    }

    pub async fn get_account_info(
        &self,
        account_type: &ConnectType,
    ) -> Result<OAuthAccountInfo, Box<dyn std::error::Error>> {
        match account_type {
            ConnectType::Google => Ok(OAuthAccountInfo::from_google(
                self.get_google_user_info().await?,
            )),
            ConnectType::Facebook => Ok(OAuthAccountInfo::from_facebook(
                self.get_facebook_user_info().await?,
            )),
        }
    }
}
