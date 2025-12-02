use sqlx::PgPool;

pub async fn create_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(db_url).await?;
    Ok(pool)
}
