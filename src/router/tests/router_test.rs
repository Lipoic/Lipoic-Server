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
        r#"{"code":1,"data":"hello world!"}"#
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
