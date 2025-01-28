#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    _entities::products,
    models::products::ProductResult,
    response::{ResponseAPI, TableResponseAPI},
};

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn get_all_products(
    State(_ctx): State<AppContext>,
    params: axum::extract::Query<std::collections::HashMap<String, Option<u64>>>,
) -> Result<Response> {
    let limit = params.get("limit").and_then(|v| *v).unwrap_or(10);
    let offset = params.get("offset").and_then(|v| *v).unwrap_or(0);

    let products = match products::Model::get_all_products(&_ctx.db, limit, offset).await {
        Ok(products) => products,
        Err(err) => {
            tracing::error!("Error fetching products: {}", err);
            return Ok(format::json(()).into_response());
        }
    };

    Ok(Json(TableResponseAPI::success(
        products
            .products
            .into_iter()
            .map(|product| ProductResult {
                id: product.id,
                name: product.name,
                description: product.description,
                price: product.price,
                stock: product.stock,
                img_url: product.img_url,
                variant_name: product.variant_name,
                category_name: product.category_name,
                created_at: product.created_at,
                updated_at: product.updated_at,
            })
            .collect(),
        products.total_count,
        "success",
    ))
    .into_response())
}

#[debug_handler]
pub async fn get_product_by_id(
    State(_ctx): State<AppContext>,
    Path(id): Path<u64>,
) -> Result<Response> {
    let product = match products::Model::get_product_by_id(&_ctx.db, id.try_into().unwrap()).await {
        Ok(product) => product,
        Err(err) => {
            tracing::error!("Error fetching product: {}", err);
            return Ok(format::json(()).into_response());
        }
    };

    Ok(Json(ResponseAPI::success(
        ProductResult {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            img_url: product.img_url,
            variant_name: product.variant_name,
            category_name: product.category_name,
            created_at: product.created_at,
            updated_at: product.updated_at,
        },
        "success",
    ))
    .into_response())
}

#[debug_handler]
pub async fn delete_product_by_id(
    State(_ctx): State<AppContext>,
    Path(id): Path<u64>,
) -> Result<Response> {
    match products::Model::delete_product_by_id(&_ctx.db, id.try_into().unwrap()).await {
        Ok(_) => Ok(Json(ResponseAPI::success((), "Product deleted successfully")).into_response()),
        Err(err) => {
            tracing::error!("Error deleting product: {}", err);
            Ok(Json(ResponseAPI::<()>::error(&err.to_string())).into_response())
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/")
        .add("/", get(index))
        .add("/all", get(get_all_products))
        .add("/:id", get(get_product_by_id))
        .add("/:id", delete(delete_product_by_id))
}
