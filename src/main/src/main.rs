#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = router::rocket(false).await.launch().await?;

    Ok(())
}
