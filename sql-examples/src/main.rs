mod like;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
struct Expense {
    id: Uuid,
    deposits: i32,
    withdrawals: i32,
}

async fn select(pool: &Pool<Postgres>) {
    let rows = sqlx::query_as!(Expense, "select * from expenses")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}

async fn insert(pool: &Pool<Postgres>) {
    sqlx::query("insert into expenses (deposits, withdrawals) values (0, 0)")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("insert into expenses values ('00000000-0000-0000-0000-000000000000', 0, 0)")
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
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // insert(&pool).await;

    // update(&pool).await;

    // delete(&pool).await;

    select(&pool).await;

    like::main();
}
