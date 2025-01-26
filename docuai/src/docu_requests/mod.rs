//pub mod schema_builder;

pub mod requests;
//pub mod responses;
pub mod schema;
pub mod schemas;
pub mod sysinsts;

use google_generative_ai_rs::v1::{
    api::PostResult,
    gemini::request::{GenerationConfig, Request},
};

use schemas::{
    Abstract, AdditionalInstruction, AlternativeTitles, ProjectAppendices, ProjectConclusion,
    ProjectContent, ProjectDetails, ProjectInfo, ProjectReferences, ProjectStructure,
};

use serde::de::DeserializeOwned;

use crate::error::DocuError;

pub enum DocuRequest<'a> {
    AlterTitles(String),
    ProjectInfo(String),
    ProjectContent(&'a ProjectInfo),
    ProjectStructure(&'a ProjectInfo, &'a ProjectContent),
    AdditionalInstruction(&'a ProjectInfo, &'a ProjectContent, &'a ProjectStructure),
    Abstract(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
    ),
    ProjectDetails(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a Abstract,
    ),
    ProjectConclusion(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a Abstract,
        &'a ProjectDetails,
    ),

    ProjectReferences(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a Abstract,
        &'a ProjectDetails,
        &'a ProjectConclusion,
    ),

    ProjectAppendices(
        &'a ProjectInfo,
        &'a ProjectContent,
        &'a ProjectStructure,
        &'a AdditionalInstruction,
        &'a Abstract,
        &'a ProjectDetails,
        &'a ProjectConclusion,
        &'a ProjectReferences,
    ),
}

impl DocuRequest<'_> {
    #[must_use]
    pub fn to_config(&self) -> GenerationConfig {
        match self {
            Self::AlterTitles(..) => schema::get_gen_config(AlternativeTitles::to_json()),

            Self::ProjectInfo(..) => schema::get_gen_config(ProjectInfo::to_json()),

            Self::ProjectContent(..) => schema::get_gen_config(ProjectContent::to_json()),

            Self::ProjectStructure(..) => schema::get_gen_config(ProjectStructure::to_json()),

            Self::AdditionalInstruction(..) => {
                schema::get_gen_config(AdditionalInstruction::to_json())
            }

            Self::Abstract(..) => schema::get_gen_config(Abstract::to_json()),

            Self::ProjectDetails(..) => schema::get_gen_config(ProjectDetails::to_json()),

            Self::ProjectConclusion(..) => schema::get_gen_config(ProjectConclusion::to_json()),

            Self::ProjectReferences(..) => schema::get_gen_config(ProjectReferences::to_json()),

            Self::ProjectAppendices(..) => schema::get_gen_config(ProjectAppendices::to_json()),
        }
    }

    #[must_use]
    pub fn to_system_instruction(&self) -> String {
        match self {
            Self::AlterTitles(..) => sysinsts::ALTER_TITLES_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectInfo(..) => sysinsts::PROJECT_INFO_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectContent(..) => sysinsts::PROJECT_CONTENT_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectStructure(..) => {
                sysinsts::PROJECT_STRUCTURE_SYSTEM_INSTRUCTION.to_string()
            }

            Self::AdditionalInstruction(..) => {
                sysinsts::ADDITIONAL_INSTRUCTION_SYSTEM_INSTRUCTION.to_string()
            }

            Self::Abstract(..) => sysinsts::ABSTRACT_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectDetails(..) => sysinsts::PROJECT_DETAILS_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectConclusion(..) => {
                sysinsts::PROJECT_CONCLUSION_SYSTEM_INSTRUCTION.to_string()
            }

            Self::ProjectReferences(..) => {
                sysinsts::PROJECT_REFERENCES_SYSTEM_INSTRUCTION.to_string()
            }

            Self::ProjectAppendices(..) => {
                sysinsts::PROJECT_APPENDICES_SYSTEM_INSTRUCTION.to_string()
            }
        }
    }

    /// # Errors
    /// ```rust
    /// DocuError::NoTextFoundInResponse | DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it will return an error if it couldn't get the text from inside of the Gemini response,
    /// or couldn't convert the response to what's specifed at the ``R`` generic
    pub async fn to_response<R>(&self, post_result: PostResult) -> Result<R, DocuError>
    where
        R: DeserializeOwned,
    {
        let text = crate::get_text(post_result)
            .await
            .ok_or(DocuError::NoTextFoundInResponse)?;

        let parsed_json = serde_json::from_str(&text)?;
        Ok(parsed_json)
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
    pub fn to_request(&self) -> Result<Request, DocuError> {
        match self {
            Self::AlterTitles(orig_title) => Ok(requests::get_request(
                orig_title.to_owned(),
                self.to_config(),
                self.to_system_instruction(),
            )),

            Self::ProjectInfo(chosen_title) => {
                let message = format!("title: {chosen_title}");
                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectContent(project_info) => {
                let message = ProjectInfo::message(project_info)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectStructure(project_info, project_content) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::AdditionalInstruction(project_info, project_content, project_structure) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::Abstract(
                project_info,
                project_content,
                project_structure,
                additional_instruction,
            ) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?
                    + &AdditionalInstruction::message(additional_instruction)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectDetails(
                project_info,
                project_content,
                project_structure,
                additional_instruction,
                r#abstract,
            ) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?
                    + &AdditionalInstruction::message(additional_instruction)?
                    + &Abstract::message(r#abstract)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectConclusion(
                project_info,
                project_content,
                project_structure,
                additional_instruction,
                r#abstract,
                project_details,
            ) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?
                    + &AdditionalInstruction::message(additional_instruction)?
                    + &Abstract::message(r#abstract)?
                    + &ProjectDetails::message(project_details)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectReferences(
                project_info,
                project_content,
                project_structure,
                additional_instruction,
                r#abstract,
                project_details,
                project_conclusion,
            ) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?
                    + &AdditionalInstruction::message(additional_instruction)?
                    + &Abstract::message(r#abstract)?
                    + &ProjectDetails::message(project_details)?
                    + &ProjectConclusion::message(project_conclusion)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectAppendices(
                project_info,
                project_content,
                project_structure,
                additional_instruction,
                r#abstract,
                project_details,
                project_conclusion,
                project_references,
            ) => {
                let message = ProjectInfo::message(project_info)?
                    + &ProjectContent::message(project_content)?
                    + &ProjectStructure::message(project_structure)?
                    + &AdditionalInstruction::message(additional_instruction)?
                    + &Abstract::message(r#abstract)?
                    + &ProjectDetails::message(project_details)?
                    + &ProjectConclusion::message(project_conclusion)?
                    + &ProjectReferences::message(project_references)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }
        }
    }
}
