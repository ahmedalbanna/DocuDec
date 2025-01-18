#[cfg(test)]
pub mod tests {
    use super::super::get_client;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_empty_project_conclusion() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/project-conclusion",
                json!({
                    "project_info": "",
                    "project_content": "",
                    "project_structure": "",
                    "additional_instructions": "",
                    "abstract": "",
                    "project_details": ""
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    #[tokio::test]
    async fn test_partial_project_conclusion() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/project-conclusion",
                json!({
                    "project_structure": {

                    },
                    "project_info": {

                    },
                    "project_content": {
                        "contributions": [
                            "hey"
                        ]
                    },
                    "additional_instruction": {

                    },
                    "abstract": {},
                    "project_details": {}
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    #[tokio::test]
    async fn test_valid_project_conclusion() {
        let client = get_client();

        let project_content_json =
            crate::json_templates::project_content::camel_case::PROJECT_CONTENT_TEMPLATE;
        let project_info_json =
            crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;

        let project_structure_json =
            crate::json_templates::project_structure::camel_case::PROJECT_STRUCTURE_TEMPLATE;

        let additional_instruction_json =
            crate::json_templates::additional_instruction::camel_case::ADDITIONAL_INSTRUCTION_TEMPLATE;

        let abstract_json = crate::json_templates::r#abstract::camel_case::ABSTRACT_TEMPLATE;

        let project_details_json =
            crate::json_templates::project_details::camel_case::PROJECT_DETAILS_TEMPLATE;

        let project_info: Value =
            serde_json::from_str(project_info_json).expect("Failed to parse json to ProjectInfo");

        let project_structure: Value = serde_json::from_str(project_structure_json)
            .expect("Failed to parse json to ProjectStructure");

        let project_content: docuai::docu_requests::schemas::ProjectContent =
            serde_json::from_str(project_content_json)
                .expect("Failed to parse JSON to ProjectContent");

        let additional_instruction: docuai::docu_requests::schemas::AdditionalInstruction =
            serde_json::from_str(additional_instruction_json)
                .expect("Failed to parse JSON to AdditionalInstruction");

        let r#abstract: docuai::docu_requests::schemas::Abstract =
            serde_json::from_str(abstract_json).expect("Failed to parse Abstract");

        let project_details: docuai::docu_requests::schemas::ProjectDetails =
            serde_json::from_str(project_details_json).unwrap();

        let res = client
            .do_post(
                "/gen/project-details",
                json!({
                    "project_info": project_info,
                    "project_content": project_content,
                    "project_structure": project_structure,
                    "additional_instruction": additional_instruction,
                    "abstract": r#abstract,
                    "project_details": project_details,
                }),
            )
            .await
            .unwrap();

        println!("{:#?}", res.json_body());

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None)
    }
}
