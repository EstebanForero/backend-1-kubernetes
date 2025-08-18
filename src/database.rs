use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    async fn new(url: String) -> Result<Self> {
        let pool = PgPoolOptions::new().connect(&url).await?;

        Ok(Self { pool })
    }
}
