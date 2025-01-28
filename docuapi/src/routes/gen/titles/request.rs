use super::super::validators::validate_title::validate_title_content;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate)]
pub struct TitleRequest {
    #[validate(
        length(
            min = 30,
            max = 80,
            message = "Title must be between 30 and 80 characters"
        ),
        custom(function = "validate_title_content")
    )]
    pub title: String,
}
