use crate::dao::db::DB;
use sqlx::{mysql::MySqlRow, FromRow, Row};
use crate::AppError;

#[derive(Debug, FromRow)]
pub struct HtUser {
    pub id: u64,
    pub mobile: String,
}

impl HtUser {
    fn new(id: u64, mobile: String) -> HtUser {
        HtUser {
            id: id,
            mobile: mobile,
        }
    }
}
// 创建用户
pub async fn user_create(db: &DB) -> Result<Vec<HtUser>, AppError> {
    let ht_users: Vec<HtUser> = sqlx::query(r#"SELECT * FROM `ht_user`"#)
        .map(|row: MySqlRow| HtUser::new(row.get::<u64, _>("id"), row.get::<String, _>("mobile")))
        .fetch_all(db)
        .await?;
    Ok(ht_users)
}

// // 更新用户
// pub fn user_update(db: DB) {}

// 根据用户Id查询用户
pub fn user_get_by_id(db: DB) {}

// // 查询用户列表
// pub fn user_list(db: DB) {}
