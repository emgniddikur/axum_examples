use sqlx::{Pool, Postgres};

// 重複行を削除する
// データの種類を取得したい場合に使う
pub async fn main(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!("SELECT DISTINCT name FROM expenses")
        .fetch_all(pool)
        .await?;
    println!("{:#?}", rows);

    Ok(())
}
