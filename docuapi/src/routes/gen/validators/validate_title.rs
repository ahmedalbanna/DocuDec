use actix_web::web::Json;
use serde::Deserialize;
use tracing::error;
use validator::{Validate, ValidationError};

use super::super::error::GenError;

/// # Errors
/// ```rust
/// [
///     "Title cannot be purely numeric",
///     "Excessive special characters in title".
///     "Invalid title pattern"
/// ]
/// ```
pub fn validate_title_content(title: &str) -> Result<(), ValidationError> {
    // Prevent purely numeric titles
    if title.chars().all(char::is_numeric) {
        return Err(ValidationError::new("Title cannot be purely numeric"));
    }

    // Prevent titles with excessive special characters
    //
    // the title it self is in between 3 and 150, so no loss would happen
    #[allow(clippy::cast_precision_loss)]
    let special_char_ratio = title
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
        .count() as f64
        / title.len() as f64;

    if special_char_ratio > 0.2 {
        return Err(ValidationError::new(
            "Excessive special characters in title",
        ));
    }

    // Prevent common spam or placeholder titles
    let banned_patterns = ["test", "placeholder", "example", "untitled", "new project"];

    if banned_patterns
        .iter()
        .any(|&pattern| title.to_lowercase().contains(pattern))
    {
        return Err(ValidationError::new("Invalid title pattern"));
    }

    Ok(())
}

/// # Errors
/// ```rust
/// // a key value validation error
/// // e.g { "title": "blahblah" }
/// GenError::ValidationError(String)
/// ```
pub fn validate_title_request<T>(payload: &'_ Json<T>) -> Result<(), GenError>
where
    T: for<'de> Deserialize<'de> + Validate,
{
    payload.validate().map_err(|validation_errors| {
        let error_details = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let error_messages = errors
                    .iter()
                    .map(|e| e.message.clone().unwrap_or_default().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("{field}: {error_messages}")
            })
            .collect::<Vec<_>>()
            .join("; ");

        error!("Validation failed: {}", error_details);

        GenError::ValidationError(error_details)
    })?;

    Ok(())
}
