#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = router::rocket().await.launch().await?;

    Ok(())
}
