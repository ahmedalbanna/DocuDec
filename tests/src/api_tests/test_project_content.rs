//use crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;
use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use super::super::{CLIENT, PROJECT_ID};
    //use super::*;
    use serde_json::json;

    pub async fn test_empty_project_content() {
        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

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
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_partial_project_content() {
        let client = &*CLIENT;

        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

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
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed to get the json value");
        let json_body = json_body["error"].to_string();

        assert!(&json_body.contains("JSON Deserialization Error"));
    }

    pub async fn test_valid_project_content() {
        let client = &*CLIENT;

        let project_id = {
            let project_id = PROJECT_ID.read().unwrap();
            (*project_id).clone().unwrap()
        };

        //let json_data = PROJECT_INFO_TEMPLATE;

        //let project_info: serde_json::Value =
        //    serde_json::from_str(json_data).expect("Failed to parse json to ProjectInfo");
        //

        let res = client
            .do_post(
                "/gen/project-content",
                json!({
                    "project_id": project_id,
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Failed to get the json value");

        assert_eq!(json_body.get("error"), None);
    }
}
