mod select;
mod where_;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
struct Expense {
    id: Uuid,
    name: String,
    deposits: i32,
    withdrawals: i32,
}

async fn insert(pool: &Pool<Postgres>) {
    sqlx::query("insert into expenses (name, deposits, withdrawals) values ('a', 0, 0)")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("insert into expenses values ('00000000-0000-0000-0000-000000000000', 'b', 0, 0)")
        .execute(pool)
        .await
        .unwrap();
}

async fn update(pool: &Pool<Postgres>) {
    sqlx::query("update expenses set deposits = 0")
        .execute(pool)
        .await
        .unwrap();
}

async fn delete(pool: &Pool<Postgres>) {
    sqlx::query("delete from expenses")
        .execute(pool)
        .await
        .unwrap();
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // insert(&pool).await;

    // update(&pool).await;

    // delete(&pool).await;

    // select::select(&pool).await;

    // where_::like::main(&pool).await;
    // where_::between::main(&pool).await;
    // where_::in_::main(&pool).await;
    // where_::any_all::main(&pool).await;

    // select::distinct::main(&pool).await?;
    // select::order_by::main(&pool).await?;

    Ok(())
}
