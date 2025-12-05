use crate::entities::stream::Stream;
use sqlx::PgPool;

pub async fn insert_stream(pool: &PgPool, stream: &Stream) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO streams (
            stream_id,
            user_id,
            user_login,
            user_name,
            game_id,
            game_name,
            stream_type,
            title,
            viewer_count,
            started_at,
            language, 
            thumbnail_url,
            tags,
            is_mature
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::timestamptz, $11, $12, $13, $14)
        ON CONFLICT stream_id UPDATE
        "#,
    )
    .bind(&stream.stream_id)
    .bind(&stream.user_id)
    .bind(&stream.user_login)
    .bind(&stream.user_name)
    .bind(&stream.game_id)
    .bind(&stream.game_name)
    .bind(&stream.stream_type)
    .bind(&stream.title)
    .bind(&stream.viewer_count)
    .bind(&stream.started_at)
    .bind(&stream.language)
    .bind(&stream.thumbnail_url)
    .bind(&stream.tags)
    .bind(&stream.is_mature)
    .execute(pool)
    .await?;

    Ok(())
}
