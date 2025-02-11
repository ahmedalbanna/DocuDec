use super::super::error::GenError;

const MAX_TITLE_CHARS: usize = 80;
const MIN_TITLE_CHARS: usize = 20;

/// # Errors
/// ```rust
/// // a key value validation error
/// // e.g { "title": "blahblah" }
/// GenError::ValidationError(String)
///
/// // possible errors
/// [
///     "Title cannot be purely numeric",
///     "Excessive special characters in title",
///     "Title must be between 20 and 80 characters"
/// ]
/// ```
pub fn validate_title_content(title: &str) -> Result<(), GenError> {
    let title_len = title.len();

    if !(MIN_TITLE_CHARS..=MAX_TITLE_CHARS).contains(&title_len) {
        return Err(GenError::ValidationError(
            "Title must be between 20 and 80 characters".into(),
        ));
    }
    // Prevent purely numeric titles
    if title.chars().all(char::is_numeric) {
        return Err(GenError::ValidationError(
            "Title cannot be purely numeric".into(),
        ));
    }

    // Prevent titles with excessive special characters
    //
    // the title it self is in between 20 and 80, so no loss would happen
    #[allow(clippy::cast_precision_loss)]
    let special_char_ratio = title
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
        .count() as f64
        / title.len() as f64;

    if special_char_ratio > 0.2 {
        return Err(GenError::ValidationError(
            "Excessive special characters in title".into(),
        ));
    }

    //// Prevent common spam or placeholder titles
    //let banned_patterns = ["test", "placeholder", "example", "untitled", "new project"];
    //
    //if banned_patterns
    //    .iter()
    //    .any(|&pattern| title.to_lowercase().contains(pattern))
    //{
    //    return Err(ValidationError::new("Invalid title pattern"));
    //}

    Ok(())
}
