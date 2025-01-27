use std::sync::RwLock;

use once_cell::sync::Lazy;

//mod test_abstract;
//mod test_additional_instruction;
mod test_auth;
//mod test_project_appendices;
//mod test_project_conclusion;
//mod test_project_content;
//mod test_project_info;
//mod test_project_references;
//mod test_project_structure;
mod test_title;

pub static CLIENT: Lazy<httpc_test::Client> = Lazy::new(|| {
    dotenv::dotenv().expect("Failed to find a .env file, make sure it's in the root directory");

    let port = std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number");

    httpc_test::new_client(format!("http://localhost:{}", port))
        .expect("Failed to create a new test http client")
});

pub static LOGGED_IN: RwLock<bool> = RwLock::new(false);

pub fn wait_until_logged_in() {
    loop {
        if let Ok(state) = LOGGED_IN.read() {
            if *state {
                break;
            }
        }
    }
}
