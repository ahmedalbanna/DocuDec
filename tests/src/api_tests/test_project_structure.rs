use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use serde_json::json;

    use crate::api_tests::{read_project_id, CLIENT};

    pub async fn test_empty_project_structure() {
        let client = &CLIENT;

        let project_id = read_project_id();

        let res = client
            .do_post(
                "/gen/project-structure",
                json!({
                    "project_id": project_id,
                    "project_content": "",
                    "project_info": ""
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("project-structure failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_partial_project_structure() {
        let client = &CLIENT;

        let project_id = read_project_id();

        let res = client
            .do_post(
                "/gen/project-structure",
                json!({
                    "project_id": project_id,
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
            .unwrap_or_else(|err| panic!("project-structure failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_valid_project_structure() {
        let client = &CLIENT;

        let project_id = read_project_id();

        let res = client
            .do_post(
                "/gen/project-structure",
                json!({
                    "project_id": project_id,
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("project-structure failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None);
    }
}
