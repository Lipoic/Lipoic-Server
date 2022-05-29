use util::oauth::{OAuthData, OauthAccountType};

const CLIENT_ID: &str = "123";
const CLIENT_SECRET: &str = "abc";

#[test]
fn google_oauth2_get_auth_url_test() {
    let oauth = OAuthData::new(
        OauthAccountType::Google,
        CLIENT_SECRET.to_string(),
        CLIENT_ID.to_string(),
        "http://127.0.0.1:8000".to_string(),
        "/",
    );

    assert_eq!("https://accounts.google.com/o/oauth2/auth?client_id=123&response_type=code&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email&redirect_uri=http%3A%2F%2F127.0.0.1%3A8000%2F", oauth.get_auth_url())
}
