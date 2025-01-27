use dotenv::dotenv;
use sqlx::{postgres::PgPool, Error as SqlxError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPool::connect(&db_url)
        .await
        .expect("couldn't connect to database");

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
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- each project is linked to a user
                title VARCHAR(150) UNIQUE NOT NULL, 
                state VARCHAR(50) DEFAULT null CHECK (state IN ('project_info', 'project_content', 'project_structure', 'additional_instruction', 'project_abstract', 'project_references', 'project_conclusion', 'project_appendices', 'chapter_1', 'chapter_2', 'chapter_3', 'chapter_4', 'chapter_5', 'chapter_6', 'chapter_7', 'chapter_8', 'chapter_9', 'chapter_10', 'chapter_11')),  -- tracks what's the latest report is
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
                type VARCHAR(50) NOT NULL CHECK (type IN ('project_info', 'project_content', 'project_structure', 'additional_instruction', 'project_abstract', 'project_references', 'project_conclusion', 'project_appendices', 'chapter_1', 'chapter_2', 'chapter_3', 'chapter_4', 'chapter_5', 'chapter_6', 'chapter_7', 'chapter_8', 'chapter_9', 'chapter_10', 'chapter_11')), 
                content JSONB NOT NULL, -- a json object of the report response coming from the AI
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )
            "#
    )
    .execute(pool)
    .await?;

    // Add indexes for performance
    sqlx::query!(r#"CREATE INDEX IF NOT EXISTS idx_projects_user_id ON projects (user_id);"#)
        .execute(pool)
        .await?;
    sqlx::query!(r#"CREATE INDEX IF NOT EXISTS idx_reports_project_id ON reports (project_id);"#)
        .execute(pool)
        .await?;
    sqlx::query!(r#"CREATE INDEX IF NOT EXISTS idx_reports_type ON reports (type);"#)
        .execute(pool)
        .await?;

    Ok(())
}
