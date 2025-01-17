use dotenv::dotenv;
use sqlx::{postgres::PgPool, Error as SqlxError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    // Get the DATABASE_URL from environment variables or default to an example
    let db_url = std::env::var("DATABASE_URL")?;

    // Connect to MySQL using sqlx
    let pool = PgPool::connect(&db_url).await.unwrap();

    // Create the database and tables
    create_tables(&pool).await?;

    Ok(())
}

async fn create_tables(pool: &PgPool) -> Result<(), SqlxError> {
    sqlx::query!(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(100) UNIQUE NOT NULL,
                password VARCHAR(255) NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
            CREATE TABLE IF NOT EXISTS projects (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                title VARCHAR(150) NOT NULL,
                information JSONB,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
            CREATE TABLE IF NOT EXISTS reports (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                content JSONB,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
