use fileshare::startup::startup;

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
    startup().await?.await?;
    Ok(())
}
