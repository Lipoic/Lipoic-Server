use database::model::auth::user::ConnectType;
use util::oauth::OAuthData;

const CLIENT_ID: &str = "123";
const CLIENT_SECRET: &str = "abc";

#[test]
fn google_oauth2_get_auth_url_test() {
    let client_secret = &CLIENT_SECRET.to_string();
    let client_id = &CLIENT_ID.to_string();
    let issuer = &"http://127.0.0.1:8000".to_string();

    let oauth = OAuthData::new(
        &ConnectType::Google,
        client_secret,
        client_id,
        issuer,
        "/login",
    );

    assert_eq!("https://accounts.google.com/o/oauth2/auth?client_id=123&response_type=code&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email&redirect_uri=http%3A%2F%2F127.0.0.1%3A8000%2Flogin", oauth.get_auth_url())
}

#[test]
fn facebook_oauth2_get_auth_url_test() {
    let client_secret = &CLIENT_SECRET.to_string();
    let client_id = &CLIENT_ID.to_string();
    let issuer = &"http://127.0.0.1:8000".to_string();

    let oauth = OAuthData::new(
        &ConnectType::Facebook,
        client_secret,
        client_id,
        issuer,
        "/login",
    );

    assert_eq!("https://www.facebook.com/dialog/oauth?client_id=123&response_type=code&scope=public_profile%2Cemail&redirect_uri=http%3A%2F%2F127.0.0.1%3A8000%2Flogin", oauth.get_auth_url())
}
