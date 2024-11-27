#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{_entities::products, models::products::ProductResult, response::ResponseAPI};

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[allow(clippy::module_name_repetitions)]
pub async fn get_all_products(
    State(_ctx): State<AppContext>,
    params: axum::extract::Query<std::collections::HashMap<String, Option<u64>>>,
) -> Json<ResponseAPI<Vec<ProductResult>>> {
    let limit = params.get("limit").and_then(|v| v.clone()).unwrap_or(10);
    let offset = params.get("offset").and_then(|v| v.clone()).unwrap_or(0);

    let products = match products::Model::get_all_products(&_ctx.db, limit, offset).await {
        Ok(products) => Some(products),
        Err(err) => {
            tracing::error!("Error fetching products: {}", err);
            None
        }
    };

    Json(ResponseAPI {
        data: products,
        success: true,
        status_code: 200,
        message: "success".to_string(),
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/")
        .add("/", get(index))
        .add("/all", get(get_all_products))
}
