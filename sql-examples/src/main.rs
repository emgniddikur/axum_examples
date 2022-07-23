use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let row = sqlx::query!("select * from expenses")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", row);
}
