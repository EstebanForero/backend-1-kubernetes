use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
}

impl Product {
    pub fn from_name(name: String) -> Self {
        let product_uuid = Uuid::new_v4();

        Self {
            id: product_uuid,
            name,
        }
    }
}

impl From<ProductCreator> for Product {
    fn from(value: ProductCreator) -> Self {
        Product::from_name(value.name)
    }
}

#[derive(Deserialize)]
pub struct ProductCreator {
    pub name: String,
}
