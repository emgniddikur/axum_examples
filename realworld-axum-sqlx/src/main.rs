use clap::Parser;
use sqlx::{migrate, postgres::PgPoolOptions};

use realworld_axum_sqlx::{config::Config, http};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();
    println!("{}", config.database_url);

    let pool = PgPoolOptions::new().connect(&config.database_url).await?;

    migrate!().run(&pool).await?;

    http::serve(config, pool).await?;

    Ok(())
}
