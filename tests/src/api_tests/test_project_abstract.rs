use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use crate::api_tests::{CLIENT, PROJECT_ID};

    use serde_json::json;

    pub async fn test_empty_project_abstract() {
        let client = &CLIENT;

        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

        let res = client
            .do_post(
                "/gen/project-abstract",
                json!({
                    "project_id": project_id,
                    "project_info": "",
                    "project_content": "",
                    "project_structure": "",
                    "additional_instructions": "",
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_partial_project_abstract() {
        let client = &CLIENT;

        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

        let res = client
            .do_post(
                "/gen/project-abstract",
                json!({
                    "project_id": project_id,
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

                    }
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_valid_project_abstract() {
        let client = &CLIENT;

        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

        let res = client
            .do_post(
                "/gen/project-abstract",
                json!({
                    "project_id": project_id,
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None)
    }
}
