use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL").expect("Failed to get the DATABASE_URL env, make sure to set it")
});

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRECT").expect("Failed to get the JWT_SECRET env, make sure to set it")
});

pub static ARGON_CONFIG: Lazy<argon2::Argon2> = Lazy::new(argon2::Argon2::default);
