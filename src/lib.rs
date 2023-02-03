use std::net::SocketAddr;
mod routes;

/// # Panics
/// Will panic if the server could not start
pub async fn run() {
    let app = routes::routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server_future = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;

    match server_future {
        Ok(_) => println!("Server started."),
        Err(error) => panic!("Server could not start: {error:?}"),
    };
}
