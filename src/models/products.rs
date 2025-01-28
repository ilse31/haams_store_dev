use super::_entities::products::{ActiveModel, Entity};
use sea_orm::{entity::prelude::*, QuerySelect};
use serde::{Deserialize, Serialize};

pub type Products = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

//result Model custom for Product
#[must_use]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResult {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub img_url: Option<String>,
    pub variant_name: Option<String>,
    pub category_name: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsResponse {
    pub products: Vec<ProductResult>,
    pub total_count: u64,
}

impl super::_entities::products::Model {
    #[allow(clippy::return_self_not_must_use)]
    #[must_use]
    pub fn to(self) -> Self {
        Self {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price,
            stock: self.stock,
            img_url: self.img_url,
            variant_id: self.variant_id,
            category_id: self.category_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    #[must_use]
    pub fn from(self) -> super::products::ActiveModel {
        super::products::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            name: sea_orm::ActiveValue::Set(self.name),
            description: sea_orm::ActiveValue::Set(self.description),
            price: sea_orm::ActiveValue::Set(self.price),
            stock: sea_orm::ActiveValue::Set(self.stock),
            img_url: sea_orm::ActiveValue::Set(self.img_url),
            variant_id: sea_orm::ActiveValue::Set(self.variant_id),
            category_id: sea_orm::ActiveValue::Set(self.category_id),
            created_at: sea_orm::ActiveValue::Set(self.created_at),
            updated_at: sea_orm::ActiveValue::Set(self.updated_at),
        }
    }

    /// Retrieves all products with their associated categories and variants.
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Number of products to skip
    ///
    /// # Returns
    /// A vector of `ProductResult` containing product details with category and variant names
    ///
    /// # Errors
    /// Returns a `DbErr` if:
    /// * Failed to fetch products from the database
    /// * Failed to fetch related categories or variants
    /// * Database connection error occurs
    pub async fn get_all_products(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<ProductsResponse, DbErr> {
        // Fetch products with their related categories and variants
        let products_with_categories = Entity::find()
            .find_with_related(super::_entities::categories::Entity)
            .limit(Some(limit))
            .offset(Some(offset.saturating_sub(1)))
            .all(db)
            .await?;

        let mut product_results = Vec::new();

        for (product, categories) in products_with_categories {
            let variants = product
                .find_related(super::_entities::variants::Entity)
                .all(db)
                .await?;

            // Map product, category, and variant into ProductResult
            let product_result = ProductResult {
                id: product.id,
                name: product.name,
                description: product.description,
                price: product.price,
                stock: product.stock,
                img_url: product.img_url,
                category_name: categories.first().and_then(|c| c.name.clone()),
                variant_name: variants.first().and_then(|v| v.name.clone()),
                created_at: product.created_at,
                updated_at: product.updated_at,
            };

            product_results.push(product_result);
        }

        //total count
        let total_count = Entity::find().count(db).await?;

        if product_results.is_empty() {
            return Err(DbErr::Custom("No products found".to_string()));
        }

        Ok(ProductsResponse {
            products: product_results,
            total_count,
        })
    }

    pub async fn get_product_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<ProductResult, DbErr> {
        let product = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Product with id {} not found",
                id
            )))?;

        let categories = product
            .find_related(super::_entities::categories::Entity)
            .all(db)
            .await?;

        let variants = product
            .find_related(super::_entities::variants::Entity)
            .all(db)
            .await?;

        // Map product, category, and variant into ProductResult
        Ok(ProductResult {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            img_url: product.img_url,
            category_name: categories.first().and_then(|c| c.name.clone()),
            variant_name: variants.first().and_then(|v| v.name.clone()),
            created_at: product.created_at,
            updated_at: product.updated_at,
        })
    }

    pub async fn delete_product_by_id(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        let product = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Product with id {} not found",
                id
            )))?;

        product.delete(db).await?;

        Ok(())
    }
}
