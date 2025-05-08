use axum::{
    extract::{Request, State},
    middleware, RequestExt, Router,
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use axum_github_oauth::{AuthAction, GithubOauthService, User};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use clap::Parser;

/// Command line arguments
#[derive(Parser)]
struct Args {
    /// Bind address in format IP:PORT
    #[clap(long, default_value = "0.0.0.0:3000")]
    bind_address: String,
}

/// Middleware to check if the user is authorized.
async fn auth(
    State(state): State<GithubOauthService>,
    mut request: Request,
) -> Result<Request, AuthAction> {
    let path = request.uri().path();
    // Combine library's public paths with application-specific public paths
    let app_public_paths = ["/", "/static"];

    if state.is_public(path) || app_public_paths.iter().any(|p|
        path == *p || path.starts_with(&format!("{}/", p))
    ) {
        Ok(request)
    } else {
        request
            .extract_parts_with_state::<User, GithubOauthService>(&state)
            .await
            .map(|_user| request)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt::init(); // https://docs.rs/tower-http/0.6.2/tower_http/trace/index.html#example

    let oauth_service = GithubOauthService::new(None).unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/protected", get(protected_handler))
        .merge(oauth_service.router())
        .layer(middleware::map_request_with_state(
            oauth_service.clone(),
            auth,
        ))
        .layer(TraceLayer::new_for_http())
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found_handler)
        .with_state(oauth_service);

    let listener = match tokio::net::TcpListener::bind(&args.bind_address).await {
        Ok(listener) => listener,
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            eprintln!("Error: Address already in use");
            std::process::exit(1);
        }
        Err(e) => return Err(e.into()),
    };

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub async fn home_handler() -> Html<String> {
    Html(String::from("Hello public"))
}

pub async fn protected_handler(user: User) -> Html<String> {
    Html(format!("Hello {}, here is the s3cret", user.login))
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(String::from("not found")))
}
