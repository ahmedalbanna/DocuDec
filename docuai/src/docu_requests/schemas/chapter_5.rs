use crate::{
    docu_requests::{context::ProcessRequest, sysinsts, RequestSchema},
    DocuRequest,
};

#[derive(Default, Debug)]
pub struct Chapter5;

impl RequestSchema for Chapter5 {
    fn to_json_value() -> serde_json::Value {
        serde_json::Value::Null
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::CHAPTER_5_PLAIN_SYSTEM_INSTRUCTION
    }
}

impl ProcessRequest for Chapter5 {
    fn process_request(
        request: &crate::DocuRequest<Self>,
        message: &mut String,
        system_instruction: &mut String,
        is_plain: bool,
    ) -> Result<(), crate::DocuError>
    where
        Self: Sized,
    {
        match request {
            DocuRequest {
                filled_info: Some(info),
                ..
            } => {
                let markdown_info =
                    json2arkdown::MarkdownRenderer::default().render(&serde_json::to_value(info)?);

                message.push_str(&format!("project info: {markdown_info}"));

                system_instruction.push_str(Self::to_system_instruction(is_plain));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
