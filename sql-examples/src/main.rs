use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(Debug)]
struct Expense {
    id: Uuid,
    deposits: i32,
    withdrawals: i32,
}

#[async_std::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let rows = sqlx::query_as!(Expense, "select * from expenses")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}
