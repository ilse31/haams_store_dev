#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241124_095156_categories;
mod m20241124_095241_variants;
mod m20241124_100557_products;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241124_095156_categories::Migration),
            Box::new(m20241124_095241_variants::Migration),
            Box::new(m20241124_100557_products::Migration),
        ]
    }
}
