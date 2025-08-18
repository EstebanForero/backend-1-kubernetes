use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions, prelude::FromRow};
use uuid::Uuid;

use crate::entities::Product;

#[derive(Clone)]
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

    pub async fn create_product(&self, product: Product) -> Result<()> {
        sqlx::query!(
            "INSERT INTO product (product_id, product_name) VALUES ($1, $2)",
            product.id,
            product.name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_products(&self) -> Result<Vec<Product>> {
        let products = sqlx::query_as!(ProductRow, "SELECT * FROM product")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|product_row| product_row.into())
            .collect();

        Ok(products)
    }
}

#[derive(FromRow)]
struct ProductRow {
    product_id: Uuid,
    product_name: String,
}

impl Into<Product> for ProductRow {
    fn into(self) -> Product {
        Product {
            id: self.product_id,
            name: self.product_name,
        }
    }
}
