#[cfg(test)]
mod tests {

    use crate::api_tests::{wait_until_signed_up, SIGNED_UP};

    use super::super::{wait_until_server_is_up, CLIENT, LOGGED_IN, TESTS_RAN};
    use serde_json::json;

    #[tokio::test]
    async fn signup() {
        wait_until_server_is_up();

        let client = &*CLIENT;

        let res = client
            .do_post(
                "/auth/signup",
                json!({
                    "username": "abdullah",
                    "password": "123456",
                    "email": "abdu.albanna@proton.me"
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        if let Ok(mut state) = SIGNED_UP.write() {
            *state = true;
        }

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }

    #[tokio::test]
    async fn login() {
        wait_until_server_is_up();
        wait_until_signed_up();

        let client = &*CLIENT;

        let res = client
            .do_post(
                "/auth/login",
                json!({
                    "username": "abdullah",
                    "password": "123456"
                }),
            )
            .await
            .unwrap();

        assert_eq!(res.status().as_u16(), 200);

        if let Ok(mut state) = LOGGED_IN.write() {
            *state = true;
        }

        if let Ok(mut test_ran) = TESTS_RAN.write() {
            *test_ran += 1;
        }
    }
}
