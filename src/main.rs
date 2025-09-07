use fileshare::startup::startup;

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
    startup().await?.await?;
    Ok(())
}

async fn hello_world() -> &'static str{
    "Hello World".as_ref()
}
