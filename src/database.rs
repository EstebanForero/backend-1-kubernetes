use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub async fn new(url: String) -> Result<Self> {
        let pool = PgPoolOptions::new().connect(&url).await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        Ok(sqlx::migrate!("./migrations").run(&self.pool).await?)
    }
}
