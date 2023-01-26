use axum::Router;
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::build());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
