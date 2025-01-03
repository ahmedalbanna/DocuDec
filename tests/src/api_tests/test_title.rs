#[cfg(test)]
pub mod tests {
    use super::super::get_client;
    use docuai::docu_requests::responses::alter_titles::Titles;
    use serde_json::json;

    #[tokio::test]
    async fn test_empty_title() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/titles",
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

        assert!(error_json.contains("The title was not provided"))
    }

    #[tokio::test]
    async fn test_valid_title() {
        let client = get_client();

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": "Rust programming language"
                }),
            )
            .await
            .unwrap();

        println!("{:#?}", res.json_body().unwrap());

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res
            .json_body_as::<Titles>()
            .expect("Failed convert the return JSON back to a Title struct");

        assert!(!json_body.recommendation.is_empty());
    }
}
