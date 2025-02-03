use std::{process::Child, sync::RwLock};

use dotenv::dotenv;
use once_cell::sync::Lazy;

pub mod run_server;
pub mod test_additional_instruction;
pub mod test_auth;
pub mod test_project_abstract;
pub mod test_project_appendices;
pub mod test_project_conclusion;
pub mod test_project_content;
pub mod test_project_info;
pub mod test_project_references;
pub mod test_project_structure;
pub mod test_title;

pub static CLIENT: Lazy<httpc_test::Client> = Lazy::new(|| {
    dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number");

    httpc_test::new_client(format!("http://localhost:{}", port))
        .expect("Failed to create a new test http client")
});

pub static SERVER_PROCESS: RwLock<Option<Child>> = RwLock::new(None);

pub static PROJECT_ID: RwLock<Option<String>> = RwLock::new(None);
