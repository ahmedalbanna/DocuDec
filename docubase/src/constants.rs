use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    let url = std::env::var("DATABASE_URL")
        .expect("Failed to get the DATABASE_URL env, make sure to set it");
    let last_slash_index = url.rfind('/').expect("there is no / in DATABASE_URL?");

    let db_name = &url[1 + last_slash_index..];

    let base_url = &url[..last_slash_index];

    if std::env::var("TEST_MODE").unwrap_or_default() == "1" {
        format!("{base_url}/docudec_test")
    } else {
        format!("{base_url}/{db_name}")
    }
});

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRECT").expect("Failed to get the JWT_SECRET env, make sure to set it")
});

pub static ARGON_CONFIG: Lazy<argon2::Argon2> = Lazy::new(argon2::Argon2::default);
