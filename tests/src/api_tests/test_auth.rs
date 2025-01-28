#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::{CLIENT, LOGGED_IN};

    #[tokio::test]
    async fn login() {
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

        let mut state = LOGGED_IN.write().unwrap();
        *state = true;
    }
}
