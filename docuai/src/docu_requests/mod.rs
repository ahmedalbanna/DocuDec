//pub mod schema_builder;

pub mod requests;
//pub mod responses;
pub mod config;
pub mod schemas;
pub mod sysinsts;

use google_generative_ai_rs::v1::{
    api::PostResult,
    gemini::request::{GenerationConfig, Request},
};

use schemas::{
    AdditionalInstruction, ProjectAbstract, ProjectAppendices, ProjectConclusion, ProjectContent,
    ProjectInfo, ProjectReferences, ProjectStructure,
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::DocuError;

pub trait ToMessage {
    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if ``serde_json`` failed to convert the
    /// provided instance to json
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError>;
}

pub trait RequestSchema {
    #[must_use]
    fn to_json_value() -> Value;

    #[must_use]
    fn to_system_instruction<'a>(is_plain: bool) -> &'a str;
}

pub trait DocuComponent: RequestSchema + ToMessage {}

pub enum DocuRequest<'a> {
    AlterTitles(&'a str),
    ProjectInfo(&'a str),
    ProjectContent(&'a ProjectInfo),
    ProjectStructure(&'a ProjectInfo, &'a ProjectContent),
    AdditionalInstruction(&'a ProjectInfo, &'a ProjectContent, &'a ProjectStructure),
    ProjectAbstract(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
    ),
    ProjectConclusion(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a ProjectAbstract,
    ),

    ProjectReferences(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a ProjectAbstract,
        &'a ProjectConclusion,
    ),

    ProjectAppendices(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a ProjectAbstract,
        &'a ProjectConclusion,
        &'a ProjectReferences,
    ),
}

impl DocuRequest<'_> {
    /// # Errors
    /// ```rust
    /// DocuError::NoTextFoundInResponse | DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it will return an error if it couldn't get the text from inside of the Gemini response,
    /// or couldn't convert the response to what's specifed at the ``R`` generic
    pub fn to_response<R>(&self, post_result: PostResult) -> Result<R, DocuError>
    where
        R: DeserializeOwned,
    {
        crate::get_text(post_result)
            .map(|s| serde_json::from_str(&s).map_err(DocuError::JsonConversion))
            .ok_or(DocuError::NoTextFoundInResponse)?
    }

    // TODO: reduce the lines
    #[allow(clippy::too_many_lines)]
    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if ``serde_json`` failed to convert the
    /// provided instance to json
    pub fn to_request(
        &self,
        config: GenerationConfig,
        is_plain: bool,
    ) -> Result<Request, DocuError> {
        let mut message = String::new();
        let mut system_instruction = String::new();

        match self {
            Self::AlterTitles(orig_title) => {
                message.push_str(orig_title);
            }

            Self::ProjectInfo(chosen_title) => {
                message.push_str(&format!("title: {chosen_title}"));
            }

            _ => self.write_components(&mut message, &mut system_instruction, is_plain)?,
        };

        Ok(requests::get_request(message, config, system_instruction))
    }

    #[inline]
    fn write_components(
        &self,
        writer: &mut String,
        system_instruction: &mut String,
        is_plain: bool,
    ) -> Result<(), DocuError> {
        let components: (&[&dyn ToMessage], &str) = match *self {
            Self::ProjectContent(info) => {
                (&[info], ProjectContent::to_system_instruction(is_plain))
            }
            Self::ProjectStructure(info, content) => (
                &[info, content],
                ProjectStructure::to_system_instruction(is_plain),
            ),
            Self::AdditionalInstruction(info, content, structure) => (
                &[info, content, structure],
                AdditionalInstruction::to_system_instruction(is_plain),
            ),

            Self::ProjectAbstract(info, content, structure, add_info) => (
                &[info, content, structure, add_info],
                ProjectAbstract::to_system_instruction(is_plain),
            ),

            Self::ProjectConclusion(info, content, structure, add_info, p_abstract) => (
                &[info, content, structure, add_info, p_abstract],
                ProjectConclusion::to_system_instruction(is_plain),
            ),

            Self::ProjectReferences(info, content, structure, add_info, p_abstract, conclusion) => {
                (
                    &[info, content, structure, add_info, p_abstract, conclusion],
                    ProjectReferences::to_system_instruction(is_plain),
                )
            }

            Self::ProjectAppendices(
                info,
                content,
                structure,
                add_info,
                p_abstract,
                conclusion,
                references,
            ) => (
                &[
                    info, content, structure, add_info, p_abstract, conclusion, references,
                ],
                ProjectAppendices::to_system_instruction(is_plain),
            ),

            Self::AlterTitles(..) | Self::ProjectInfo(..) => (&[], ""),
        };

        system_instruction.push_str(components.1);

        components
            .0
            .iter()
            .try_for_each(|c| c.write_message(writer))
    }
}
