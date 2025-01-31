use crate::{
    constants::{ARGON_CONFIG, JWT_SECRET, JWT_TOKEN_EXPIRE_DAYS},
    error::DocuBaseError,
    models::Claims,
};
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// # Errors
/// should only return an error when hashing the password fails, it will return
/// ```rust
/// DocuBaseError::PasswordHashingError
/// ```
/// containing the error message from the argon2 lib
///
///
pub fn hash_password(password: &str) -> Result<String, DocuBaseError> {
    let argon2 = &*ARGON_CONFIG;
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| DocuBaseError::PasswordHashingError(err.to_string()))?;

    Ok(password_hash.to_string())
}

/// # Errors
/// should only return an error when hashing the password fails, it will return
/// ```rust
/// DocuBaseError::PasswordHashingError(...)
/// ```
/// containing the error message from the argon2 lib
///
///
pub fn verify_password(password: &str, hash: &str) -> Result<bool, DocuBaseError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|err| DocuBaseError::PasswordHashingError(err.to_string()))?;

    let argon2 = &*ARGON_CONFIG;

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// FIXME: you should properly fix this
#[allow(clippy::cast_sign_loss)]
/// # Errors
/// it will return
/// ```rust
/// DocuBaseError::JwtError(...)
/// ```
/// containing the jwt encode error
///
/// # Panics
///
/// this will panic if the ``checked_add_signed`` failed, this should not happen
///
pub fn generate_jwt(user_id: &str) -> Result<String, DocuBaseError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(JWT_TOKEN_EXPIRE_DAYS))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as u64,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(DocuBaseError::JwtError)
}

/// # Errors
/// it will return
/// ```rust
/// DocuBaseError::JwtError(...)
/// ```
/// containing the jwt encode error
pub fn validate_jwt(token: &str) -> Result<String, DocuBaseError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "sub"]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )
    .map_err(DocuBaseError::JwtError)?;

    Ok(token_data.claims.sub)
}
