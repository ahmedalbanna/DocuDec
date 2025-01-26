use once_cell::sync::Lazy;

pub static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var("PORT")
        .expect("Failed to get the PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the PORT as a number")
});

pub static ADDRESS: Lazy<String> = Lazy::new(|| {
    std::env::var("ADDRESS").expect("Failed to get the ADDRESS env, make sure to set it")
});

pub const JWT_COOKIE: &str = "jwt_token";
