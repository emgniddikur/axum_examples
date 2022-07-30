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

// 複数指定
async fn multiple(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM expenses ORDER BY deposits DESC, withdrawals DESC")
        .fetch_all(pool)
        .await?;
    println!("{:#?}", rows);

    Ok(())
}

// 列番号を指定　select命令に記述した順に1から数える
async fn column_number(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!("SELECT deposits, withdrawals FROM expenses ORDER BY 1 DESC, 2 DESC")
        .fetch_all(pool)
        .await?;
    println!("{:#?}", rows);

    Ok(())
}

pub async fn main(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // asc(pool).await?;

    // desc(pool).await?;

    // multiple(pool).await?;

    // column_number(pool).await?;

    Ok(())
}
