use std::sync::LazyLock;

pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number")
});

pub static ADDRESS: LazyLock<String> = LazyLock::new(|| {
    std::env::var("ADDRESS").expect("Failed to get the ADDRESS env, make sure to set it")
});

pub const JWT_COOKIE: &str = "jwt_token";

pub const API_TIMEOUT: u64 = 100;
