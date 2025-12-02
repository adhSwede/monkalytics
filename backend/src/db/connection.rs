use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

pub async fn create_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let max_retries = 5;
    let retry_delay = Duration::from_secs(2);

    for attempt in 1..=max_retries {
        match PgPool::connect(db_url).await {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                if attempt == max_retries {
                    eprintln!(
                        "Failed to connect to database after {} attempts",
                        max_retries
                    );
                    return Err(e);
                }
                eprintln!(
                    "Database connection attempt {} failed: {}. Retrying in {:?}...",
                    attempt, e, retry_delay
                );
                sleep(retry_delay).await;
            }
        }
    }

    unreachable!()
}
