use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::debug;

use crate::{
    error::{Error, Result},
    store::{new_db_pool, Db},
    utils::{b32_hex, b64u},
};

#[derive(Clone)]
pub struct ModelController {
    db: Db,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelController { db })
    }

    pub(crate) fn db(&self) -> &Db {
        &self.db
    }
}

#[derive(Clone, Debug)]
pub struct FoodModelController;

#[derive(Debug, Serialize)]
pub struct FoodToCreate {
    pub food_name: String,
    pub category: String,
    pub stocks: i32,
    pub price: f32,
    pub total_quantity: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct FoodToSelect {
    pub cid: String,
    pub mid: String,
    pub id: i64,
    pub stamp_code: String,
    pub food_name: String,
    pub category: String,
    pub stocks: i32,
    pub price: f64,
    pub total_quantity: i32,
    pub created_date: String,
}

#[derive(Debug, Serialize, FromRow)]
struct FoodsToReturn {
    id: i64,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OneFoodToSelect {
    pub cid: String,
    pub mid: String,
    pub id: i64,
    pub stamp_code: String,
    pub food_name: String,
    pub category: String,
    pub stocks: i32,
    pub price: f64,
    pub total_quantity: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct FoodToUpdate {
    pub id: i64,
    pub stocks: Option<i32>,
    pub price: Option<f32>,
    pub total_quantity: Option<i32>,
}

impl FoodModelController {
    pub async fn create(mm: ModelController, data: FoodToCreate) -> Result<i64> {
        debug!("{:<12} - create", "HANDLER");

        let query = "insert into foods_table (cid, mid, stamp_code, food_name, category, stocks, price, total_quantity) values ($1,$2,$3,$4,$5,$6,$7,$8) returning id";
        let cid = b64u().unwrap();
        let mid = b64u().unwrap();
        let stamp_code = b32_hex().unwrap();

        let FoodToCreate {
            food_name,
            category,
            stocks,
            price,
            total_quantity,
        } = data;
        let db = mm.db();

        match sqlx::query_as::<_, FoodsToReturn>(query)
            .bind(cid)
            .bind(mid)
            .bind(stamp_code)
            .bind(food_name)
            .bind(category)
            .bind(stocks)
            .bind(price)
            .bind(total_quantity)
            .fetch_one(db)
            .await
        {
            Ok(food) => Ok(food.id),
            Err(err) => {
                debug!("{:<12} - Create failed - error {err:?}", "ERROR_CONTROLLER");
                Err(Error::CreateFailed(err.to_string()))
            }
        }
    }

    pub async fn select(mm: ModelController) -> Result<Vec<FoodToSelect>> {
        debug!("{:<12} - select", "HANDLER");

        let query = "select cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date from foods_table where food_status != 'removed' order by ctime desc";
        let db = mm.db();

        match sqlx::query_as::<_, FoodToSelect>(query).fetch_all(db).await {
            Ok(foods) => Ok(foods),
            Err(err) => {
                debug!("{:<12} - select handler error", "ERROR_CONTROLLER");
                Err(Error::SelectFailed(err.to_string()))
            }
        }
    }

    pub async fn get_by_id(mm: ModelController, id: i64) -> Result<OneFoodToSelect> {
        debug!("{:<12} - get_by_id", "HANDLER");

        let query = "select cid, mid, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date from foods_table where id = $1";
        let db = mm.db();

        match sqlx::query_as::<_, OneFoodToSelect>(query)
            .bind(id)
            .fetch_one(db)
            .await
        {
            Ok(food) => Ok(food),
            Err(err) => {
                debug!("{:<12} - get_by_id error", "ERROR_CONTROLLER");

                Err(Error::FoodIdNotFound(err.to_string()))
            }
        }
    }

    pub async fn get_by_stamp_code(
        mm: ModelController,
        stamp_code: String,
    ) -> Result<OneFoodToSelect> {
        debug!("{:<12} - get_by_stamp_code", "HANDLER");

        let query = "select cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date from foods_table where stamp_code = $1";
        let db = mm.db();

        match sqlx::query_as::<_, OneFoodToSelect>(query)
            .bind(stamp_code)
            .fetch_one(db)
            .await
        {
            Ok(food) => Ok(food),
            Err(err) => {
                debug!("{:<12} - get_by_stamp_code error", "ERROR_CONTROLLER");

                Err(Error::FoodStampCodeNotFound(err.to_string()))
            }
        }
    }

    pub async fn update(mm: ModelController, data: FoodToUpdate) -> Result<FoodToSelect> {
        debug!("{:<12} - update handler", "HANDLER");

        let db = mm.db();

        match (data.stocks, data.price, data.total_quantity) {
            (Some(stocks), Some(price), Some(total_quantity)) => {
                let query = "update foods_table set stocks = $1, price = $2, total_quantity = $3 where id = $4 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(stocks)
                    .bind(price)
                    .bind(total_quantity)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (Some(stocks), Some(price), _) => {
                let query = "update foods_table set stocks = $1, price = $2 where id = $3 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(stocks)
                    .bind(price)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (Some(stocks), _, Some(total_quantity)) => {
                let query = "update foods_table set stocks = $1, total_quantity = $2 where id = $3 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(stocks)
                    .bind(total_quantity)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (Some(stocks), _, _) => {
                let query = "update foods_table set stocks = $1 where id = $2 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(stocks)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (_, Some(price), Some(total_quantity)) => {
                let query = "update foods_table set price = $1, total_quantity = $2 where id = $3 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(price)
                    .bind(total_quantity)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (_, Some(price), _) => {
                let query = "update foods_table set price = $1 where id = $2 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(price)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            (_, _, Some(total_quantity)) => {
                let query = "update foods_table set total_quantity = $1 where id = $2 returning cid, mid, id, stamp_code, food_name, category, stocks, price, total_quantity, to_char(ctime, 'Month DD, YYYY') as created_date";

                match sqlx::query_as::<_, FoodToSelect>(query)
                    .bind(total_quantity)
                    .bind(data.id)
                    .fetch_one(db)
                    .await
                {
                    Ok(food_updated) => Ok(food_updated),
                    Err(err) => {
                        debug!("{:<12} - update handler error", "ERROR_CONTROLLER");

                        Err(Error::UpdateFailed(err.to_string()))
                    }
                }
            }
            _ => Err(Error::FoodIdNotFound(data.id.to_string())),
        }
    }

    pub async fn delete(mm: ModelController, id: i64) -> Result<String> {
        debug!("{:<12} - delete handler", "HANDLER");

        let query = "update foods_table set food_status = 'removed' where id = $1";
        let db = mm.db();

        match sqlx::query(query).bind(id).execute(db).await {
            Ok(_) => Ok(String::from("Removed food successfully")),
            Err(err) => {
                debug!("{:<12} - delete handler error", "ERROR_CONTROLLER");
                Err(Error::DeleteFailed(err.to_string()))
            }
        }
    }
}
