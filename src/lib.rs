use std::net::SocketAddr;

mod routes;

pub async fn run() {
    let app = routes::build();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
