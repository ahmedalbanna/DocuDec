use collect_tests::collect_tests;

#[collect_tests]
mod tests {
    use super::super::CLIENT;
    use serde_json::json;

    pub async fn test_empty_title() {
        //let model = actix_web::web::Data::new(std::sync::Arc::new(docuai::get_model().unwrap()));
        //let g = docuapi::routes::gen::titles::handler::generate_titles;
        //
        //let app =
        //    actix_web::test::init_service(actix_web::App::new().app_data(model).service(g)).await;
        //let req = actix_web::test::TestRequest::post()
        //    .uri("/titles")
        //    .cookie(actix_web::cookie::Cookie::new("jwt_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzNzNlYjY3Mi04N2ExLTQ5YWItYTU0Yi1iMzMyMTNmZmIwNTciLCJleHAiOjE3Mzg2ODE4NzF9.LkvsRv7zQv4rGdY6RQHZmVsw8SW0ut59ZYkTvSGbc2Y"))
        //    .set_json(json!({
        //        "title": ""
        //    }))
        //    .to_request();
        //
        //let resp = test::call_service(&app, req).await;
        //let s = resp.status();
        //
        //let bj = actix_web::test::read_body(resp).await;
        //
        //dbg!(bj);
        //
        //assert_eq!(s, StatusCode::BAD_REQUEST);

        let client = &*CLIENT;

        let res = client
            .do_post(
                "/gen/titles",
                json!({
                    "title": ""
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("titles failed, error: {err}"));

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
            .unwrap_or_else(|err| panic!("titles failed, error: {err}"));

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
            .unwrap_or_else(|err| panic!("titles failed, error: {err}"));

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
                    "title": "@".repeat(40)
                }),
            )
            .await
            .unwrap_or_else(|err| panic!("titles failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 400);

        let json_body = res.json_body().expect("Failed get a json body");
        let error_json = json_body["error"]
            .as_str()
            .expect("Failed to get the error value");

        dbg!(&error_json);

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
            .unwrap_or_else(|err| panic!("titles failed, error: {err}"));

        assert_eq!(res.status().as_u16(), 200);

        let json_body = res
            .json_body_as::<docuai::docu_requests::schemas::AlternativeTitles>()
            .expect("Failed convert the return JSON back to a Title struct");

        assert!(!json_body.recommendation.is_empty());
    }
}
