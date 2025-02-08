use std::{
    process::Child,
    sync::{RwLock, RwLockWriteGuard},
};

use dotenv::dotenv;
use std::sync::LazyLock;

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

pub static CLIENT: LazyLock<httpc_test::Client> = LazyLock::new(|| {
    dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number");

    httpc_test::new_client(format!("http://localhost:{port}"))
        .expect("Failed to create a new test http client")
});

pub static SERVER_PROCESS: RwLock<Option<Child>> = RwLock::new(None);

pub static PROJECT_ID: RwLock<Option<String>> = RwLock::new(None);

pub static USER_COOKIE: RwLock<Option<String>> = RwLock::new(None);

pub fn read_project_id() -> String {
    let project_id = PROJECT_ID
        .read()
        .unwrap_or_else(|err| panic!("failed to read PROJECT_ID, error: {err}"));
    (*project_id).clone().expect("PROJECT_ID is empty")
}

pub fn write_project_id<'a>() -> RwLockWriteGuard<'a, Option<String>> {
    match PROJECT_ID.write() {
        Ok(project_id) => project_id,
        Err(err) => panic!("failed to get a writeable PROJECT_ID, error: {err}"),
    }
}
