use sqlx::{Pool, Postgres};

pub async fn main(pool: &Pool<Postgres>) {
    let rows = sqlx::query!("select * from expenses where name like '%a%'")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", rows);
}
