use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub mobile: String,
    pub real_name: String,
    pub invite_code: String,
    pub password: String,
    pub status: u8,
}
