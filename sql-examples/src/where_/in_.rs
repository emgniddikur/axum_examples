use sqlx::{Pool, Postgres};

pub async fn main(pool: &Pool<Postgres>) {
    let rows = sqlx::query!("SELECT * FROM expenses WHERE deposits IN (0, 1)")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);

    let rows = sqlx::query!("SELECT * FROM expenses WHERE deposits NOT IN (0, 1)")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}
