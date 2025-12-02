use sqlx::PgPool;

pub async fn fetch_and_store_games(
    pool: PgPool,
    client_id: &str,
    access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
}
