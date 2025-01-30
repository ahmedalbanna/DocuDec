use super::CLIENT;
use serde_json::json;

pub async fn signup() {
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
}

pub async fn login() {
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
}
