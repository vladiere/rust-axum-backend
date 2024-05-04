use crate::{
    ctx::Ctx,
    model::{Error, ModelManager, Result},
};
use sqlx::{postgres::PgRow, FromRow};

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    unimplemented!()
}
