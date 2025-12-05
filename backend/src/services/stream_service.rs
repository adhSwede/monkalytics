use crate::api::twitch::fetch_top_streams;
use crate::entities::stream::Stream;
use crate::repositories::stream_repository::insert_stream;
use chrono::Local;
use sqlx::PgPool;

pub async fn fetch_and_store_streams(
    pool: &PgPool,
    client_id: &str,
    access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting fetch_and_store_streams...");

    let mut cursor: Option<String> = None;
    let mut total_stored = 0;
    let max_pages = 200; // Stop after x pages

    for page in 1..=max_pages {
        println!("Fetching page {}/{}", page, max_pages);

        let streams_response =
            fetch_top_streams(client_id, access_token, cursor.as_deref()).await?;

        let streams_count = streams_response.data.len();

        // If viewers < 5 stop.
        if let Some(last_stream) = streams_response.data.last() {
            if last_stream.viewer_count < 5 {
                println!("Reached streams with <5 viewers, stopping");
                break;
            }
        }

        // Insert streams
        for dto in streams_response.data {
            let stream = Stream {
                id: None,
                stream_id: dto.id,
                user_id: dto.user_id,
                user_login: dto.user_login,
                user_name: dto.user_name,
                game_id: dto.game_id,
                game_name: dto.game_name,
                stream_type: dto.stream_type,
                title: dto.title,
                viewer_count: dto.viewer_count,
                started_at: dto.started_at,
                language: dto.language,
                thumbnail_url: dto.thumbnail_url,
                tag_ids: dto.tag_ids.map(|v| v.join(",")).or(Some(String::new())),
                tags: dto.tags.map(|v| v.join(",")).or(Some(String::new())),
                is_mature: dto.is_mature,
                polled_at: None,
            };
            insert_stream(pool, &stream).await?;
        }

        total_stored += streams_count;

        // Get next cursor
        cursor = streams_response.pagination.and_then(|p| p.cursor);

        if cursor.is_none() {
            println!("No more pages");
            break;
        }
    }

    println!(
        "[{}] ✓ Stored {} streams total",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        total_stored
    );
    Ok(())
}
