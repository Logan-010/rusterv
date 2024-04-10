use axum::Router;
use std::{env, process::exit};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("USAGE: {} <PORT> <DIR>", &args[0]);
        exit(0);
    }

    let port: u16 = args[1]
        .trim()
        .parse()
        .expect("Make sure <PORT> argument is a port number!");

    let path = &args[2];

    println!("Listening on:\n\thttp://localhost:{port}");

    let routes = Router::new()
        .nest_service("/", ServeDir::new(path))
        .layer(CompressionLayer::new());
    let listener = TcpListener::bind(("localhost", port))
        .await
        .expect("Failed to bind to port!");

    axum::serve(listener, routes)
        .await
        .expect("Failed to serve server!");
}
