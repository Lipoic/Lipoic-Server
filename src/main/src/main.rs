use router;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(router::stage())
        .launch()
        .await?;

    Ok(())
}