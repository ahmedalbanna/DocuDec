#[cfg(test)]
pub mod tests {
    use crate::api_tests::TESTS_RAN;

    use super::super::{wait_until_logged_in, CLIENT};
    use docuai::docu_requests::schemas::AlternativeTitles;
    use serde_json::json;

    #[tokio::test]
    async fn test_empty_title() {
        wait_until_logged_in();

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

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }

    #[tokio::test]
    async fn test_big_title() {
        wait_until_logged_in();

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

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }

    #[tokio::test]
    async fn test_all_numbers() {
        wait_until_logged_in();

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

        println!("{:#?}", res.json_body());

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed get a json body");
        let error_json = json_body["error"]
            .as_str()
            .expect("Failed to get the error value");

        assert!(error_json.contains("Title cannot be purely numeric"));

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }

    #[tokio::test]
    async fn test_all_special_characters() {
        wait_until_logged_in();

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

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }

    #[tokio::test]
    async fn test_valid_title() {
        wait_until_logged_in();

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
            .json_body_as::<AlternativeTitles>()
            .expect("Failed convert the return JSON back to a Title struct");

        assert!(!json_body.recommendation.is_empty());

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }
}
