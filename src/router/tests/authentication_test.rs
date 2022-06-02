use rocket::http::Status;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn google_oauth_url() {
    let client = Client::tracked(router::rocket(true).await)
        .await
        .expect("valid rocket instance");
    let req = client
        .get("/api/authentication/google/url?redirect_uri=http://127.0.0.1/login/oauth/google");
    let response = rocket::tokio::join!(req.clone().dispatch());

    assert_eq!(response.0.status(), Status::Ok);
    assert_eq!(
        response.0.into_string().await.unwrap(),
        r#"{"code":200,"message":"Ok.","data":{"url":"https://accounts.google.com/o/oauth2/auth?client_id=&response_type=code&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email&redirect_uri=http%3A%2F%2F127.0.0.1%2Flogin%2Foauth%2Fgoogle"}}"#
    );
}

#[rocket::async_test]
async fn facebook_oauth_url() {
    let client = Client::tracked(router::rocket(true).await)
        .await
        .expect("valid rocket instance");
    let req = client
        .get("/api/authentication/facebook/url?redirect_uri=http://127.0.0.1/login/oauth/facebook");
    let response = rocket::tokio::join!(req.clone().dispatch());

    assert_eq!(response.0.status(), Status::Ok);
    assert_eq!(
        response.0.into_string().await.unwrap(),
        r#"{"code":200,"message":"Ok.","data":{"url":"https://www.facebook.com/dialog/oauth?client_id=&response_type=code&scope=public_profile%2Cemail&redirect_uri=http%3A%2F%2F127.0.0.1%2Flogin%2Foauth%2Ffacebook"}}"#
    );
}
