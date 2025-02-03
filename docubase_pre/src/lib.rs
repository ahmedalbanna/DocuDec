use dotenv::dotenv;
use sqlx::{postgres::PgPool, Error as SqlxError};
use std::env;

/// # Errors
///
/// if the DATABASE_URL is not set
/// something is wrong with the database queries
///
/// # Panics
///
/// if there is no '/' in the ``DATABASE_URL``
///
/// if it couldn't connect to the database
///
#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let db_url = env::var("DATABASE_URL")?;

    let full_db_url = make_sure_db_exists(&db_url).await?;

    let pool = PgPool::connect(&full_db_url)
        .await
        .expect("Couldn't connect to the database");

    create_tables(&pool).await?;

    Ok(())
}

/// # Errors
///
/// it would return a sqlx error if something is wrong with the database
///
/// # Panics
///
/// if there is no '/' in the ``DATABASE_URL``
///
async fn make_sure_db_exists(full_db_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // db_name is what's at the end of the url
    //
    // in here, if the TEST_MODE is set to 1, then it will get docudec_test
    // else it will get the last thing after the '/' from the url, e.g
    // (postgresql://root:2003@localhost/docudec) will get the docudec
    //
    let db_name = env::var("TEST_MODE")
        .unwrap_or_default()
        .eq("1")
        .then_some("docudec_test")
        .unwrap_or_else(|| {
            full_db_url
                .split('/')
                .last()
                .expect("there was no '/' in DATABASE_URL")
        });

    // the last '/' index
    let slash_index = full_db_url
        .rfind('/')
        .expect("there was no '/' in DATABASE_URL");

    // everything up until the last '/' so we can specify our own database
    let db_url = &full_db_url[..slash_index];

    let connection = PgPool::connect(&format!("{db_url}/postgres")).await?;

    // recreate the docudec_test again to test from the beggining
    if db_name == "docudec_test" {
        sqlx::query("DROP DATABASE IF EXISTS docudec_test")
            .execute(&connection)
            .await?;
    }

    let query =
        format!("SELECT EXISTS(SELECT 1 FROM pg_catalog.pg_database WHERE datname = '{db_name}');");

    let (exists,): (bool,) = sqlx::query_as(&query).fetch_one(&connection).await?;

    if !exists {
        let create_query = format!("CREATE DATABASE {db_name};");
        sqlx::query(&create_query).execute(&connection).await?;
    }

    Ok(format!("{db_url}/{db_name}"))
}

async fn create_tables(pool: &PgPool) -> Result<(), SqlxError> {
    // Constants for table creation SQL queries
    const CREATE_USERS_TABLE: &str = "
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )";

    const CREATE_PROJECTS_TABLE: &str = "
        CREATE TABLE IF NOT EXISTS projects (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(150) UNIQUE NOT NULL, 
            state VARCHAR(50) DEFAULT NULL CHECK (state IN (
                'project_info', 'project_content', 'project_structure', 
                'additional_instruction', 'project_abstract', 'project_references',
                'project_conclusion', 'project_appendices', 'chapter_1', 'chapter_2', 
                'chapter_3', 'chapter_4', 'chapter_5', 'chapter_6', 'chapter_7', 
                'chapter_8', 'chapter_9', 'chapter_10', 'chapter_11'
            )),
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )";

    const CREATE_REPORTS_TABLE: &str = "
        CREATE TABLE IF NOT EXISTS reports (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            type VARCHAR(50) NOT NULL CHECK (type IN (
                'project_info', 'project_content', 'project_structure', 
                'additional_instruction', 'project_abstract', 'project_references',
                'project_conclusion', 'project_appendices', 'chapter_1', 'chapter_2', 
                'chapter_3', 'chapter_4', 'chapter_5', 'chapter_6', 'chapter_7', 
                'chapter_8', 'chapter_9', 'chapter_10', 'chapter_11'
            )),
            content JSONB NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )";

    // Execute SQL queries to create tables if not exists
    sqlx::query(CREATE_USERS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_PROJECTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_REPORTS_TABLE).execute(pool).await?;

    // Add indexes for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_projects_user_id ON projects (user_id);")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_reports_project_id ON reports (project_id);")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_reports_type ON reports (type);")
        .execute(pool)
        .await?;

    Ok(())
}
