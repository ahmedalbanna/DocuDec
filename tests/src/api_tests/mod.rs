mod test_abstract;
mod test_additional_instruction;
mod test_project_content;
mod test_project_info;
mod test_project_structure;
mod test_title;

pub fn get_client() -> httpc_test::Client {
    dotenv::dotenv().expect("Failed to find a .env file, make sure it's in the root directory");

    let port = std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number");

    httpc_test::new_client(format!("http://localhost:{}", port))
        .expect("Failed to create a new test http client")
}
