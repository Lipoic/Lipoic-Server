use rocket::http::Status;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn hello_test() {
    let client = Client::tracked(router::rocket(true).await)
        .await
        .expect("valid rocket instance");
    let req = client.get("/");
    let response = rocket::tokio::join!(req.clone().dispatch());
    assert_eq!(response.0.status(), Status::Ok);
    assert_eq!(
        response.0.into_string().await.unwrap(),
        r#"{"code":1,"message":"Ok.","data":"hello world!"}"#
    )
}

#[rocket::async_test]
async fn not_found_test() {
    let client = Client::tracked(router::rocket(true).await)
        .await
        .expect("valid rocket instance");
    let req = client.get("/test");
    let response = rocket::tokio::join!(req.clone().dispatch());

    assert_eq!(response.0.status(), Status::NotFound);
    assert_eq!(
        response.0.into_string().await.unwrap(),
        include_str!("../../../resources/404.html")
    );
}

#[rocket::async_test]
async fn google_oauth_url() {
    let client = Client::tracked(router::rocket(true).await)
        .await
        .expect("valid rocket instance");
    let req = client.get("/api/authentication/google/url?redirect_uri=/api/authentication/google");
    let response = rocket::tokio::join!(req.clone().dispatch());

    assert_eq!(response.0.status(), Status::Ok);
    assert_eq!(
        response.0.into_string().await.unwrap(),
        r#"{"code":1,"message":"Ok.","data":{"url":"https://accounts.google.com/o/oauth2/auth?client_id=&response_type=code&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email&redirect_uri=%2Fapi%2Fauthentication%2Fgoogle"}}"#
    );
}
