use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Products::Table)
                    .col(pk_auto(Products::Id))
                    .col(string_null(Products::Name))
                    .col(text_null(Products::Description))
                    .col(decimal_null(Products::Price))
                    .col(integer_null(Products::Stock))
                    .col(string_null(Products::ImgUrl))
                    .col(integer_null(Products::VariantId))
                    .col(integer_null(Products::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_variant")
                            .from(Products::Table, Products::VariantId)
                            .to(Variants::Table, Variants::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category")
                            .from(Products::Table, Products::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Name,
    Description,
    Price,
    Stock,
    ImgUrl,
    VariantId,
    CategoryId,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Variants {
    Table,
    Id,
}
