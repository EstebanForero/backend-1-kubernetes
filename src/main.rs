use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

use crate::{
    database::PostgresRepo,
    entities::{Product, ProductCreator},
};

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
pub mod entities;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    match dotenvy::dotenv() {
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

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/product", post(add_product).get(get_products))
        .with_state(pg_rp)
        .layer(cors);

    let ip_addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(ip_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn health_check() -> &'static str {
    "I am alive"
}

async fn add_product(
    State(pg_rp): State<PostgresRepo>,
    Json(product): Json<ProductCreator>,
) -> impl IntoResponse {
    match pg_rp.create_product(product.into()).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            error!("Error in add prodct endpoint: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_products(
    State(pg_rp): State<PostgresRepo>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match pg_rp.get_products().await {
        Ok(products) => Ok(Json(products)),
        Err(err) => {
            error!("Error in get products endpoint: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
