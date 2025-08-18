use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
struct Config {
    db_host: String,
    db_port: u16,
    db_user: String,
    db_password: String,
    db_name: String,
}

mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    match dotenvy::from_filename(".env.dev") {
        Ok(_) => info!("ENV variables loaded from the .env file"),
        Err(_) => info!(".env file doesn't exist, skipping step"),
    }

    let config: Config =
        envy::from_env().expect("Failed to load config from the environment variables");
}
