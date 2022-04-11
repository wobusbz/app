use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub type DB = Pool<MySql>;

pub async fn db_connection(databse: String) -> DB {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&databse)
        .await;
    match pool {
        Ok(p) => p,
        Err(e) => {
            panic!("mysql connection failed {}", e)
        }
    }
}
