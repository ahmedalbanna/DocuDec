use dotenv::dotenv;
use sqlx::{mysql::MySqlPool, MySql, Pool};
use std::env;

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool<MySql>,
}

impl Database {
    pub async fn create_pool() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        let pool = MySqlPool::connect(&db_url).await?;
        Ok(Database { pool })
    }
}
