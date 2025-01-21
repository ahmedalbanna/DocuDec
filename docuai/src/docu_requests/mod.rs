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
    Abstract, AdditionalInstruction, ProjectConclusion, ProjectContent, ProjectDetails,
    ProjectInfo, ProjectReferences, ProjectStructure, ABSTRACT_JSON_VALUE,
    ADDITIONALINSTRUCTION_JSON_VALUE, ALTERNATIVETITLES_JSON_VALUE, PROJECTAPPENDICES_JSON_VALUE,
    PROJECTCONCLUSION_JSON_VALUE, PROJECTCONTENT_JSON_VALUE, PROJECTDETAILS_JSON_VALUE,
    PROJECTINFO_JSON_VALUE, PROJECTREFERENCES_JSON_VALUE, PROJECTSTRUCTURE_JSON_VALUE,
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
    pub fn to_config(&self) -> GenerationConfig {
        match self {
            Self::AlterTitles(_) => {
                let json = &*ALTERNATIVETITLES_JSON_VALUE;

                schema::get_gen_config(json.clone())
            }

            Self::ProjectInfo(_) => {
                let json = &*PROJECTINFO_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectContent(_) => {
                let json = &*PROJECTCONTENT_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectStructure(_, _) => {
                let json = &*PROJECTSTRUCTURE_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::AdditionalInstruction(_, _, _) => {
                let json = &*ADDITIONALINSTRUCTION_JSON_VALUE;

                schema::get_gen_config(json.clone())
            }

            Self::Abstract(_, _, _, _) => {
                let json = &*ABSTRACT_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectDetails(_, _, _, _, _) => {
                let json = &*PROJECTDETAILS_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectConclusion(_, _, _, _, _, _) => {
                let json = &*PROJECTCONCLUSION_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectReferences(_, _, _, _, _, _, _) => {
                let json = &*PROJECTREFERENCES_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }

            Self::ProjectAppendices(_, _, _, _, _, _, _, _) => {
                let json = &*PROJECTAPPENDICES_JSON_VALUE;
                schema::get_gen_config(json.clone())
            }
        }
    }

    pub fn to_system_instruction(&self) -> String {
        match self {
            Self::AlterTitles(_) => sysinsts::ALTER_TITLES_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectInfo(_) => sysinsts::PROJECT_INFO_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectContent(_) => sysinsts::PROJECT_CONTENT_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectStructure(_, _) => {
                sysinsts::PROJECT_STRUCTURE_SYSTEM_INSTRUCTION.to_string()
            }

            Self::AdditionalInstruction(_, _, _) => {
                sysinsts::ADDITIONAL_INSTRUCTION_SYSTEM_INSTRUCTION.to_string()
            }

            Self::Abstract(_, _, _, _) => sysinsts::ABSTRACT_SYSTEM_INSTRUCTION.to_string(),

            Self::ProjectDetails(_, _, _, _, _) => {
                sysinsts::PROJECT_DETAILS_SYSTEM_INSTRUCTION.to_string()
            }

            Self::ProjectConclusion(_, _, _, _, _, _) => {
                sysinsts::PROJECT_CONCLUSION_SYSTEM_INSTRUCTION.to_string()
            }

            Self::ProjectReferences(_, _, _, _, _, _, _) => {
                sysinsts::PROJECT_REFERENCES_SYSTEM_INSTRUCTION.to_string()
            }

            Self::ProjectAppendices(_, _, _, _, _, _, _, _) => {
                sysinsts::PROJECT_APPENDICES_SYSTEM_INSTRUCTION.to_string()
            }
        }
    }

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

    pub fn to_request(&self) -> Result<Request, DocuError> {
        match self {
            Self::AlterTitles(orig_title) => Ok(requests::get_request(
                orig_title.to_owned(),
                self.to_config(),
                self.to_system_instruction(),
            )),

            Self::ProjectInfo(chosen_title) => Ok(requests::get_request(
                chosen_title.to_owned(),
                self.to_config(),
                self.to_system_instruction(),
            )),

            Self::ProjectContent(project_info) => {
                let message = serde_json::to_string(*project_info)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::ProjectStructure(project_info, project_content) => {
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}",
                    project_info, project_content
                );

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }

            Self::AdditionalInstruction(project_info, project_content, project_structure) => {
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}",
                    project_info, project_content, project_structure
                );

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
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;
                let additional_instruction = serde_json::to_string(*additional_instruction)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}\nadditionalInstructions: {:#?}",
                    project_info, project_content, project_structure, additional_instruction
                );

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
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;
                let additional_instruction = serde_json::to_string(*additional_instruction)?;
                let r#abstract = serde_json::to_string(*r#abstract)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}\nadditionalInstructions: {:#?} \nAbstract: {:#?}",
                    project_info, project_content, project_structure, additional_instruction, r#abstract
                );

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
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;
                let additional_instruction = serde_json::to_string(*additional_instruction)?;
                let r#abstract = serde_json::to_string(*r#abstract)?;
                let project_details = serde_json::to_string(*project_details)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}\nadditionalInstructions: {:#?} \nabstract: {:#?} \nprojectDetails: {:#?}",
                    project_info, project_content, project_structure, additional_instruction, r#abstract, project_details
                );

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
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;
                let additional_instruction = serde_json::to_string(*additional_instruction)?;
                let r#abstract = serde_json::to_string(*r#abstract)?;
                let project_details = serde_json::to_string(*project_details)?;
                let project_conclusion = serde_json::to_string(*project_conclusion)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}\nadditionalInstructions: {:#?} \nabstract: {:#?} \nprojectDetails: {:#?} \nprojectConclusion: {:#?}",
                    project_info, project_content, project_structure, additional_instruction, r#abstract, project_details, project_conclusion
                );

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
                let project_info = serde_json::to_string(*project_info)?;
                let project_content = serde_json::to_string(*project_content)?;
                let project_structure = serde_json::to_string(*project_structure)?;
                let additional_instruction = serde_json::to_string(*additional_instruction)?;
                let r#abstract = serde_json::to_string(*r#abstract)?;
                let project_details = serde_json::to_string(*project_details)?;
                let project_conclusion = serde_json::to_string(*project_conclusion)?;
                let project_references = serde_json::to_string(*project_references)?;

                let message = format!(
                    "projectInformation: {:#?}\nprojectContent: {:#?}\nstructureAndFormat: {:#?}\nadditionalInstructions: {:#?} \nabstract: {:#?} \nprojectDetails: {:#?} \nprojectConclusion: {:#?} \nprojectReferences: {:#?}",
                    project_info, project_content, project_structure, additional_instruction, r#abstract, project_details, project_conclusion, project_references
                );

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }
        }
    }
}
