use crate::error::ApiError;
use crate::db;
use crate::models::UserLogin;

pub async fn user_login_get_token(username: &str, password: &str) -> Result<UserLogin, ApiError> {
    let pool = db::get_db_pool()?;

    let user = sqlx::query_as!(
        UserLogin,
        "SELECT token FROM users WHERE username = ? AND password = ?",
        username,
        password
    )
    .fetch_optional(pool)
    .await?;

    match user {
        Some(user) => Ok(user),
        None => Err(ApiError::Unauthorized("Invalid username or password".into())),
    }
}