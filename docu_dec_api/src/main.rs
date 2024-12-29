use axum::Router;

mod routes;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .nest("/auth", routes::auth::routes().await)
        .nest("/gen", routes::gen::routes().await)
        .nest("/file", routes::file::route().await);

    let tcp = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Started server on: {}", tcp.local_addr().unwrap());

    axum::serve(tcp, routes.into_make_service()).await.unwrap();
}
