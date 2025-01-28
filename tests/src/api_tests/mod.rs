use std::{process::Child, sync::RwLock};

use dotenv::dotenv;
use once_cell::sync::Lazy;

//mod test_abstract;
//mod test_additional_instruction;
mod test_auth;
//mod test_project_appendices;
//mod test_project_conclusion;
//mod test_project_content;
mod test_project_info;
//mod test_project_references;
//mod test_project_structure;
mod run_server;
mod test_title;

pub static CLIENT: Lazy<httpc_test::Client> = Lazy::new(|| {
    dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number");

    httpc_test::new_client(format!("http://localhost:{}", port))
        .expect("Failed to create a new test http client")
});

pub static LOGGED_IN: RwLock<bool> = RwLock::new(false);
pub static SIGNED_UP: RwLock<bool> = RwLock::new(false);
pub static GOT_PROJECT_ID: RwLock<bool> = RwLock::new(false);
pub static SERVER_RUNNING: RwLock<bool> = RwLock::new(false);
pub static SERVER_PROCESS: RwLock<Option<Child>> = RwLock::new(None);

pub static PROJECT_ID: RwLock<Option<String>> = RwLock::new(None);

const TESTS_COUNTS: u32 = 9;
pub static TESTS_RAN: RwLock<u32> = RwLock::new(0);

pub fn wait_until_signed_up() {
    loop {
        if let Ok(state) = SIGNED_UP.read() {
            if *state {
                break;
            }
        }
    }
}

pub fn wait_until_logged_in() {
    loop {
        if let Ok(state) = LOGGED_IN.read() {
            if *state {
                break;
            }
        }
    }
}

pub fn wait_until_server_is_up() {
    loop {
        if let Ok(state) = SERVER_RUNNING.read() {
            if *state {
                break;
            }
        }
    }
}

pub fn wait_until_all_tests_complete() {
    loop {
        if let Ok(test_ran) = TESTS_RAN.read() {
            if *test_ran == TESTS_COUNTS {
                break;
            }
        }
    }
}

pub fn wait_until_got_project_id() {
    loop {
        if let Ok(state) = GOT_PROJECT_ID.read() {
            if *state {
                break;
            }
        }
    }
}
