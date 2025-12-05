use crate::api::twitch::fetch_top_streams;
use crate::entities::stream::Stream;
use crate::repositories::stream_repository::insert_stream;
use sqlx::PgPool;

pub async fn fetch_and_store_streams(
    pool: &PgPool,
    client_id: &str,
    access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting fetch_and_store_streams...");

    let streams_response = fetch_top_streams(client_id, access_token).await?;
    let streams_count = streams_response.data.len();

    println!("Fetched {} streams from API", streams_count);

    // for dto in streams_response.data {
    for (i, dto) in streams_response.data.into_iter().enumerate() {
        println!("Inserting stream {}/{}", i + 1, streams_count);

        let stream = Stream {
            id: None,
            stream_id: dto.id,
            user_id: dto.user_id,
            user_login: dto.user_login,
            user_name: dto.user_name,
            game_id: Some(dto.game_id),
            game_name: Some(dto.game_name),
            stream_type: dto.stream_type,
            title: Some(dto.title),
            viewer_count: dto.viewer_count,
            started_at: Some(dto.started_at),
            language: Some(dto.language),
            thumbnail_url: Some(dto.thumbnail_url),
            tags: Some(dto.tags.join(",")),
            is_mature: dto.is_mature,
        };

        insert_stream(pool, &stream).await?;
    }

    println!("✓ Stored {} streams", streams_count);
    Ok(())
}
