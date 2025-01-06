#[cfg(test)]
pub mod tests {
    use super::super::get_client;
    use docuai::docu_requests::responses::project_info::ProjectInfo;
    use serde_json::json;

    #[tokio::test]
    async fn test_empty_project_info_title() {
        let client = get_client();

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

        assert!(error_json.contains("Title must be between 3 and 150 characters"))
    }

    #[tokio::test]
    async fn test_valid_project_info_title() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/project-info",
                json!({
                    "title": "Rust programming language"
                }),
            )
            .await
            .unwrap();

        println!("{:#?}", res.json_body().unwrap());

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res
            .json_body_as::<ProjectInfo>()
            .expect("Failed convert the return JSON back to a ProjectInfo struct");

        println!("{:#?}", json_body);
        assert!(!json_body.project_title.is_empty());
    }
}
