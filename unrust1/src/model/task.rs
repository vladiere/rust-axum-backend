use crate::{
    ctx::Ctx,
    model::{ModelManager, Result},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: ---- Task Types

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// endregion: ---- Task Types

// region: ---- Task Backend Model Controller

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();

        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task_table (title) values ($1) returning id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }
}
// endregion: ---- Task Backend Model Controller

// region: ---- Unit testing

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // ---- Setup and Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "text_create_ok title";

        // ---- Execute
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // ---- Checking
        let (title,): (String,) = sqlx::query_as("SELECT title from task_table where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;
        assert_eq!(title, fx_title);

        // ---- Cleaning
        let count = sqlx::query("DELETE FROM task_table WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }
}

// endregion: ---- Unit testing
