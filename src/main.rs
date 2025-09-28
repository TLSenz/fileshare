use std::fmt::format;
use tokio::net::TcpListener;
use fileshare::configuration::get_config;
use fileshare::db::create_pool;
use fileshare::startup::startup;

#[tokio::main]
async fn main() -> Result<(),std::io::Error> {
    let configuration = get_config().expect("Failde to start. Could not Read Config");
    let database_url = configuration.connection_string_database();
    let pg_pool = create_pool(&database_url).await.expect("Could not get connection to database");
    let listener = TcpListener::bind(configuration.connection_string_application()).await?;
    startup(listener,pg_pool).await?.await?;
    Ok(())
}
