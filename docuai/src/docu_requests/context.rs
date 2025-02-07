use std::marker::PhantomData;

use crate::DocuError;

use super::schemas::{
    AdditionalInstruction, AlternativeTitles, ProjectAbstract, ProjectAppendices,
    ProjectConclusion, ProjectContent, ProjectInfo, ProjectReferences, ProjectStructure,
};

#[derive(Default)]
pub struct Empty;

pub trait ProcessRequest {
    fn process_request(
        request: &DocuRequest<Self>,
        message: &mut String,
        system_instruction: &mut String,
        is_plain: bool,
    ) -> Result<(), DocuError>
    where
        Self: Sized;
}

#[derive(Debug, Default)]
pub struct DocuRequest<'a, S = Empty> {
    pub(crate) titles: Option<&'a str>,
    pub(crate) info: Option<&'a str>,
    pub(crate) filled_info: Option<&'a ProjectInfo>,
    pub(crate) content: Option<&'a ProjectContent>,
    pub(crate) structure: Option<&'a ProjectStructure>,
    pub(crate) additional_instruction: Option<&'a AdditionalInstruction>,
    pub(crate) abstrak: Option<&'a ProjectAbstract>,
    pub(crate) conclusion: Option<&'a ProjectConclusion>,
    pub(crate) references: Option<&'a ProjectReferences>,
    pub(crate) appendices: Option<&'a ProjectAppendices>,
    _marker: PhantomData<S>,
}

impl<'a> DocuRequest<'a, Empty> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_info(self, info: &'a str) -> DocuRequest<'a, ProjectInfo> {
        DocuRequest {
            info: Some(info),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_filled_info(self, info: &'a ProjectInfo) -> DocuRequest<'a, ProjectContent> {
        DocuRequest {
            filled_info: Some(info),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_titles(self, titles: &'a str) -> DocuRequest<'a, AlternativeTitles> {
        DocuRequest {
            titles: Some(titles),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, ProjectContent> {
    #[must_use]
    pub fn with_content(self, content: &'a ProjectContent) -> DocuRequest<'a, ProjectStructure> {
        DocuRequest {
            filled_info: self.filled_info,
            content: Some(content),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, ProjectStructure> {
    #[must_use]
    pub fn with_structure(
        self,
        structure: &'a ProjectStructure,
    ) -> DocuRequest<'a, AdditionalInstruction> {
        DocuRequest {
            filled_info: self.filled_info,
            content: self.content,
            structure: Some(structure),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, AdditionalInstruction> {
    #[must_use]
    pub fn with_additional_instruction(
        self,
        additional_instruction: &'a AdditionalInstruction,
    ) -> DocuRequest<'a, ProjectAbstract> {
        DocuRequest {
            filled_info: self.filled_info,
            content: self.content,
            structure: self.structure,
            additional_instruction: Some(additional_instruction),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, ProjectAbstract> {
    #[must_use]
    pub fn with_project_abstract(
        self,
        abstrak: &'a ProjectAbstract,
    ) -> DocuRequest<'a, ProjectConclusion> {
        DocuRequest {
            filled_info: self.filled_info,
            content: self.content,
            structure: self.structure,
            additional_instruction: self.additional_instruction,
            abstrak: Some(abstrak),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, ProjectConclusion> {
    #[must_use]
    pub fn with_project_conclusion(
        self,
        conclusion: &'a ProjectConclusion,
    ) -> DocuRequest<'a, ProjectReferences> {
        DocuRequest {
            filled_info: self.filled_info,
            content: self.content,
            structure: self.structure,
            additional_instruction: self.additional_instruction,
            abstrak: self.abstrak,
            conclusion: Some(conclusion),
            ..Default::default()
        }
    }
}

impl<'a> DocuRequest<'a, ProjectReferences> {
    #[must_use]
    pub fn with_project_references(
        self,
        references: &'a ProjectReferences,
    ) -> DocuRequest<'a, ProjectAppendices> {
        DocuRequest {
            filled_info: self.filled_info,
            content: self.content,
            structure: self.structure,
            additional_instruction: self.additional_instruction,
            abstrak: self.abstrak,
            conclusion: self.conclusion,
            references: Some(references),
            ..Default::default()
        }
    }
}

//impl<'a> DocuRequest<'a, ProjectAppendices> {
//    #[must_use]
//    pub fn with_project_appendices(
//        self,
//        appendices: &'a ProjectAppendices,
//    ) -> DocuRequest<'a, ProjectAppendices> {
//        DocuRequest {
//            filled_info: self.filled_info,
//            content: self.content,
//            structure: self.structure,
//            additional_instruction: self.additional_instruction,
//            abstrak: self.abstrak,
//            conclusion: self.conclusion,
//            references: self.references,
//            appendices: Some(appendices),
//            ..Default::default()
//        }
//    }
//}
