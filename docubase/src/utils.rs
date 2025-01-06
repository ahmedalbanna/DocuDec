use crate::models::Claims;
use argon2::{self, Config, Variant, Version};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::error::Error;

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let password = password.as_bytes();
    let salt = "docudec-ahmed-abdullah".as_bytes();
    let config = Config {
        variant: Variant::Argon2d,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();

    Ok(hash)
}

pub fn verify_password(password: String, hash: String) -> bool {
    argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
}

pub fn generate_jwt(user_id: &str) -> Result<String, Box<dyn Error>> {
    let secret = "super-secret";

    let expiration = (chrono::Utc::now().timestamp() + 3600) as usize; // Token expires in 1 hour

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let token = encode(&Header::default(), &claims, &encoding_key)?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> Result<String, Box<dyn Error>> {
    let secret = "super-secret";

    use jsonwebtoken::{decode, DecodingKey, Validation};

    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())?;

    // Check if the token is expired
    if token_data.claims.exp < chrono::Utc::now().timestamp() as usize {
        return Err("Token has expired.".into());
    }

    Ok(token_data.claims.sub)
}
