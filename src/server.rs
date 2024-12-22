use axum::Router;
use std::env;
use std::net::SocketAddr;

pub async fn start_server(app: Router) {
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16");

    let addr = SocketAddr::new(server_host.parse().unwrap(), server_port);
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
