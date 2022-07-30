pub mod distinct;

use sqlx::{Pool, Postgres};

pub async fn select(pool: &Pool<Postgres>) {
    let rows = sqlx::query!("select * from expenses")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}
