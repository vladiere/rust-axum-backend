use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    crud_fns::{FoodModelController, FoodToCreate, FoodToUpdate, ModelController},
    error::Result,
};

pub fn routes_crud(mm: ModelController) -> Router {
    Router::new()
        .route("/api/create", post(api_create_food))
        .route("/api/update", post(api_update_food))
        .route("/api/select", get(api_select_food))
        .route("/api/select/:id", get(api_select_food_by_id))
        .route(
            "/api/select/stamp_code/:stamp_code",
            get(api_select_food_by_stamp_code),
        )
        .route("/api/delete/:id", delete(api_delete_food))
        .with_state(mm)
}

#[derive(Debug, Deserialize)]
struct CreateFoodPayload {
    food_name: String,
    category: String,
    stocks: i32,
    price: f32,
    total_quantity: i32,
}

#[derive(Debug, Deserialize)]
struct UpdateFoodPayload {
    id: i64,
    stocks: Option<i32>,
    price: Option<f32>,
    total_quantity: Option<i32>,
}

async fn api_create_food(
    State(mm): State<ModelController>,
    Json(body): Json<CreateFoodPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_create_food", "ROUTE_HANDLER");

    let CreateFoodPayload {
        food_name,
        category,
        stocks,
        price,
        total_quantity,
    } = body;

    let data = FoodToCreate {
        food_name: food_name.to_string(),
        category: category.to_string(),
        stocks,
        price,
        total_quantity,
    };

    let food_id = FoodModelController::create(mm, data).await?;
    let body = Json(json!({
        "result": {
            "message": "success",
            "status": true,
            "food_id": food_id,
        }
    }));

    Ok(body)
}

async fn api_select_food(State(mm): State<ModelController>) -> Result<Json<Value>> {
    debug!("{:<12} - api_select_food", "ROUTE_HANDLER");

    let foods = FoodModelController::select(mm).await?;
    let body = Json(json!({
        "result": {
            "data": foods,
            "status": true,
        }
    }));
    Ok(body)
}

async fn api_select_food_by_id(
    State(mm): State<ModelController>,
    Path(food_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_select_food_by_id", "ROUTE_HANDLER");

    let id = food_id;

    let food = FoodModelController::get_by_id(mm, id).await?;

    let body = Json(json!({
        "result": {
            "data": food,
            "status": true,
        }
    }));
    Ok(body)
}

async fn api_select_food_by_stamp_code(
    State(mm): State<ModelController>,
    Path(stamp_code): Path<String>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_select_food_by_stamp_code", "ROUTE_HANDLER");

    let stamp_code = stamp_code;

    let food = FoodModelController::get_by_stamp_code(mm, stamp_code).await?;

    let body = Json(json!({
        "result": {
            "data": food,
            "status": true,
        }
    }));
    Ok(body)
}

async fn api_update_food(
    State(mm): State<ModelController>,
    Json(body): Json<UpdateFoodPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_update_food", "ROUTE_HANDLER");

    let UpdateFoodPayload {
        id,
        stocks,
        price,
        total_quantity,
    } = body;
    let data = FoodToUpdate {
        id,
        stocks,
        price,
        total_quantity,
    };

    let updated_food = FoodModelController::update(mm, data).await?;

    let body = Json(json!({
        "result": {
            "data": updated_food,
            "status": true,
        }
    }));
    Ok(body)
}

async fn api_delete_food(
    State(mm): State<ModelController>,
    Path(food_id): Path<i64>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_delete_food", "ROUTE_HANDLER");

    let id = food_id;

    let food = FoodModelController::delete(mm, id).await?;

    let body = Json(json!({
        "resutl": {
            "message": food,
            "status": true,
        }
    }));
    Ok(body)
}
