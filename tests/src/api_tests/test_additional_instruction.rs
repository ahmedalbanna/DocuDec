#[cfg(test)]
pub mod tests {
    use super::super::get_client;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_empty_additional_instruction() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/additional-instructions",
                json!({
                    "project_info": "",
                    "project_content": "",
                    "project_structure": "",
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("Failed to deserialize the JSON body into the target type"));
    }

    #[tokio::test]
    async fn test_partial_project_structure() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/additional-instructions",
                json!({
                    "project_structure": {

                    },
                    "project_info": {

                    },
                    "project_content": {
                        "contributions": [
                            "hey"
                        ]
                    }
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("Failed to deserialize the JSON body into the target type"));
    }

    #[tokio::test]
    async fn test_valid_project_structure() {
        let client = get_client();

        let project_content_json =
            crate::json_templates::project_content::camel_case::PROJECT_CONTENT_TEMPLATE;
        let project_info_json =
            crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;

        let project_structure_json =
            crate::json_templates::project_structure::camel_case::PROJECT_STRUCTURE_TEMPLATE;

        let project_info: Value =
            serde_json::from_str(project_info_json).expect("Failed to parse json to ProjectInfo");

        let project_structure: Value = serde_json::from_str(project_structure_json)
            .expect("Failed to parse json to ProjectStructure");

        let project_content: docuai::docu_requests::responses::project_content::ProjectContent =
            serde_json::from_str(project_content_json)
                .expect("Failed to parse JSON to ProjectContent");

        let res = client
            .do_post(
                "/gen/additional-instructions",
                json!({
                    "project_info": project_info,
                    "project_content": project_content,
                    "project_structure": project_structure,
                }),
            )
            .await
            .unwrap();

        println!("{:#?}", res.json_body().unwrap());

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None)
    }
}
