use sqlx::{Pool, Postgres};

async fn asc(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM expenses ORDER BY deposits")
        .fetch_all(pool)
        .await?;
    println!("{:#?}", rows);

    Ok(())
}

async fn desc(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM expenses ORDER BY deposits DESC")
        .fetch_all(pool)
        .await?;
    println!("{:#?}", rows);

    Ok(())
}

pub async fn main(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    asc(pool).await?;
    desc(pool).await?;

    Ok(())
}
