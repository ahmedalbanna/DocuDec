#[cfg(test)]
pub mod tests {
    use super::super::get_client;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_empty_project_content() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_info": ""
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
    async fn test_partial_project_content() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_info": {
                        "fieldOfStudy": [
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
    async fn test_valid_project_content() {
        let client = get_client();

        let json_data = crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;

        let project_info: Value =
            serde_json::from_str(json_data).expect("Failed to parse json to ProjectInfo");

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_info": project_info
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None)
    }
}
