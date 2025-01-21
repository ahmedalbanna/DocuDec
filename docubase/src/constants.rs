use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL").expect("Failed to get the DATABASE_URL env, make sure to set it")
});

pub static JWTSALT: Lazy<String> = Lazy::new(|| {
    std::env::var("JWTSALT").expect("Failed to get the JWTSALT env, make sure to set it")
});

pub static JWTSECRECT: Lazy<String> = Lazy::new(|| {
    std::env::var("JWTSECRECT").expect("Failed to get the JWTSECRECT env, make sure to set it")
});
