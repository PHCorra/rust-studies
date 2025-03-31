use sqlx::PgPool;
use sqlx::Error;

pub async fn db_connection(db_url: &str) -> Result<PgPool, Error> {

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(db_url)
    .await?;

    println!("Connected to db! Awaiting for the functions");  

    Ok(pool);
}