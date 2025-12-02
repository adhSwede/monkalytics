use sqlx::PgPool;

async fn init_streams_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS streams (
            id SERIAL PRIMARY KEY,
            user_id TEXT NOT NULL,
            user_name TEXT NOT NULL,
            game_id TEXT,
            game_name TEXT,
            viewer_count INTEGER NOT NULL,
            language TEXT,
            title TEXT,
            started_at TIMESTAMPTZ,
            thumbnail_url TEXT,
            polled_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_streams_polled_at ON streams(polled_at)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_streams_game_id ON streams(game_id)")
        .execute(pool)
        .await?;

    println!("✓ Streams Table Initialized...");
    Ok(())
}

async fn init_games_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS games (
    game_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    box_art_url TEXT,
    igdb_id TEXT,
    last_updated TIMESTAMPTZ DEFAULT NOW()
    )"#,
    )
    .execute(pool)
    .await?;

    println!("✓ Games Table Initialized...");
    Ok(())
}

pub async fn init_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    init_streams_table(pool).await?;
    init_games_table(pool).await?;
    println!("✓ All Tables Initialized Successfully!");
    Ok(())
}
