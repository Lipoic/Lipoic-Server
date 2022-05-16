use util::oauth::GoogleOAuth;

const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";

#[test]
fn google_oauth2_get_auth_url_test() {
    let oauth = GoogleOAuth::new(
        CLIENT_SECRET.to_string(),
        CLIENT_ID.to_string(),
        "http://127.0.0.1:8000".to_string(),
        "/",
    );

    println!("{}", oauth.get_auth_url())
}
