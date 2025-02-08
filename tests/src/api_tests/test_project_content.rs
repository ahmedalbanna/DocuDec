//use crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;
use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use crate::api_tests::read_project_id;

    use super::super::CLIENT;
    use serde_json::json;

    pub async fn test_empty_project_content() {
        let project_id = read_project_id();

        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_id": project_id,
                    "project_info": ""
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("project-content failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_partial_project_content() {
        let client = &*CLIENT;

        let project_id = read_project_id();

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_id": project_id,
                    "project_info": {
                        "fieldOfStudy": [
                            "hey"
                        ]
                    }
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("project-content failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_valid_project_content() {
        let client = &*CLIENT;

        let project_id = read_project_id();

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_id": project_id,
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("project-content failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None);
    }
}
