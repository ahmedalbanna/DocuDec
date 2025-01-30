use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use docuai::docu_requests::schemas::ProjectInfo;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::super::{CLIENT, PROJECT_ID};

    #[derive(Serialize, Deserialize)]
    struct ProjectInfoRes {
        project_id: String,
        content: ProjectInfo,
    }

    pub async fn test_empty_project_info_title() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/project-info",
                json!({
                    "title": ""
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed get a json body");
        let error_json = json_body["error"]
            .as_str()
            .expect("Failed to get the error value");

        assert!(error_json.contains("Title must be between 30 and 80 characters"));
    }

    pub async fn test_valid_project_info_title() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/project-info",
                json!({
                    "title": "Rust programming language and how good it is"
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res
            .json_body_as::<ProjectInfoRes>()
            .expect("Failed convert the return JSON back to a ProjectInfo struct");

        assert!(!json_body.content.project_title.is_empty());

        if let Ok(mut project_id) = PROJECT_ID.write() {
            *project_id = Some(json_body.project_id);
        }
    }
}
