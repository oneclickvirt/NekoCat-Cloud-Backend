use actix_web::Result;
use crate::error::ApiError;
use crate::db;
use crate::models::Announcement;

pub async fn get_announcement() -> Result<Announcement, ApiError> {
    let pool = db::get_db_pool()?;    

    let announcement = sqlx::query_as!(
        Announcement,
        "SELECT id, announcement_title, announcement_body, display_portal, created_time, `type` FROM announcement ORDER BY created_time DESC LIMIT 1"
    )
    .fetch_one(pool)
    .await
    .map_err(ApiError::Database)?;

    Ok(announcement)
}
