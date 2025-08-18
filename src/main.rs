use axum::{Router, routing::get};
use serde::Deserialize;
use tracing::{error, info};

use crate::database::PostgresRepo;

#[derive(Debug, Deserialize)]
struct Config {
    db_host: String,
    db_port: u16,
    db_user: String,
    db_password: String,
    db_name: String,
    port: u16,
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

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    );

    let pg_rp = PostgresRepo::new(db_url)
        .await
        .expect("Error conncting to the database");

    if pg_rp.run_migrations().await.is_ok() {
        info!("Migrations ran succesfully");
    } else {
        error!("Error running migrations");
    }

    let app = Router::new().route("/health", get(health_check));

    let ip_addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(ip_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn health_check() -> &'static str {
    "I am alive"
}
