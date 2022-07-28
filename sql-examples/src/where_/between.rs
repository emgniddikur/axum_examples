use sqlx::{Pool, Postgres};

pub async fn main(pool: &Pool<Postgres>) {
    let rows = sqlx::query!("SELECT * FROM expenses WHERE deposits BETWEEN 1 AND 100")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}
