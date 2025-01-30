use collect_tests::collect_tests;

#[collect_tests]
mod tests {
    use super::super::CLIENT;
    use serde_json::json;

    pub async fn test_empty_title() {
        let client = &*CLIENT;

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

        assert!(error_json.contains("Title must be between 30 and 80 characters"));
    }

    pub async fn test_big_title() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": "a".repeat(150)
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

    pub async fn test_all_numbers() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": "0".repeat(40)
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed get a json body");
        let error_json = json_body["error"]
            .as_str()
            .expect("Failed to get the error value");

        assert!(error_json.contains("Title cannot be purely numeric"));
    }

    pub async fn test_all_special_characters() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": " @ ".repeat(10)
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed get a json body");
        let error_json = json_body["error"]
            .as_str()
            .expect("Failed to get the error value");

        assert!(error_json.contains("Excessive special characters in title"));
    }

    pub async fn test_valid_title() {
        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": "Rust programming language and how good it is"
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("something went wrong: {err}"));

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res
            .json_body_as::<docuai::docu_requests::schemas::AlternativeTitles>()
            .expect("Failed convert the return JSON back to a Title struct");

        assert!(!json_body.recommendation.is_empty());
    }
}
