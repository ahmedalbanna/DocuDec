use super::super::validators::validate_title::validate_title_content;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate)]
pub struct TitleRequest {
    #[serde(default)]
    #[validate(
        length(
            min = 3,
            max = 150,
            message = "Title must be between 3 and 150 characters"
        ),
        custom(function = "validate_title_content")
    )]
    pub title: String,
}
