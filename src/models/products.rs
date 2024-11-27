use super::_entities::products::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
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

impl super::_entities::products::Model {
    pub fn to(self) -> super::products::Model {
        super::products::Model {
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

    pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<ProductResult>, DbErr> {
        // Fetch products with their related categories and variants
        let products_with_categories = Entity::find()
            .find_with_related(super::_entities::categories::Entity)
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
                category_name: categories.get(0).and_then(|c| c.name.clone()), // Flatten Option
                variant_name: variants.get(0).and_then(|v| v.name.clone()),    //
                created_at: product.created_at,
                updated_at: product.updated_at,
            };

            product_results.push(product_result);
        }

        Ok(product_results)
    }
}
