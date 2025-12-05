use sqlx::PgPool;

async fn execute_query(pool: &PgPool, query: &str) -> Result<(), sqlx::Error> {
    sqlx::query(query).execute(pool).await?;
    Ok(())
}

async fn init_streams_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    execute_query(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS streams (
            id SERIAL PRIMARY KEY,
            stream_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            user_login TEXT NOT NULL,
            user_name TEXT NOT NULL,
            game_id TEXT,
            game_name TEXT,
            stream_type TEXT NOT NULL,
            title TEXT,
            viewer_count INTEGER NOT NULL,
            started_at TIMESTAMPTZ,
            language TEXT,
            thumbnail_url TEXT,
            tag_ids TEXT,
            tags TEXT,
            is_mature BOOLEAN NOT NULL DEFAULT false,
            polled_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .await?;

    execute_query(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_streams_polled_at ON streams(polled_at)",
    )
    .await?;

    execute_query(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_streams_game_id ON streams(game_id)",
    )
    .await?;

    execute_query(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_streams_stream_id ON streams(stream_id)",
    )
    .await?;

    println!("✓ Streams Table Initialized...");
    Ok(())
}

async fn init_games_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    execute_query(
        pool,
        r#"
        CREATE TABLE IF NOT EXISTS games (
            id SERIAL PRIMARY KEY,
            game_id TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            box_art_url TEXT,
            igdb_id TEXT,
            polled_at TIMESTAMPTZ DEFAULT NOW()
        )"#,
    )
    .await?;

    execute_query(
        pool,
        "CREATE INDEX IF NOT EXISTS idx_games_game_id ON games(game_id)",
    )
    .await?;

    println!("✓ Games Table Initialized...");
    Ok(())
}

pub async fn init_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    match init_streams_table(pool).await {
        Ok(_) => {}
        Err(e) => eprintln!("Streams table error: {}", e),
    }

    match init_games_table(pool).await {
        Ok(_) => {}
        Err(e) => eprintln!("Games table error: {}", e),
    }

    println!("✓✓ Table initialization complete!");
    Ok(())
}
