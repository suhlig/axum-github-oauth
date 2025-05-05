use axum::Server;
use std::net::SocketAddr;

mod error;
mod handlers;
mod lib;

use lib::{Config, GithubOauthService};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create oauth service with default config
    let service = GithubOauthService::new(None).expect("Failed to initialize OAuth service");

    // Create router
    let app = service.router();

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
