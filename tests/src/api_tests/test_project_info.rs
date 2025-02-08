use collect_tests::collect_tests;

#[collect_tests]
pub mod tests {
    use docuai::docu_requests::schemas::ProjectInfo;
    use serde_json::json;

    use crate::api_tests::write_project_id;

    use super::super::CLIENT;

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
            .unwrap_or_else(|err| panic!("project-info failed, error: {err}"));

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
            .unwrap_or_else(|err| panic!("project-info failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res.json_body().expect("Couldn't get the json body");

        let project_id = json_body
            .get("project_id")
            .expect("there was no project_id in the return json body")
            .to_string();

        let json_body: ProjectInfo = serde_json::from_value(
            json_body
                .get("content")
                .expect("there is no content in the response")
                .to_owned(),
        )
        .expect("Couldn't convert the returned content into a ProjectInfo");

        assert!(!json_body.project_title.is_empty());

        *write_project_id() = Some(project_id.trim_matches('"').to_string());
    }
}
