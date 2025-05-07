use axum::{
    extract::{Request, State},
    middleware, RequestExt, Router,
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use axum_github_oauth::{AuthAction, GithubOauthService, User};
use tower_http::services::ServeDir;

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
async fn main() {
    let oauth_service = GithubOauthService::new(None).unwrap();

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/protected", get(protected_handler))
        .merge(oauth_service.router())
        .layer(middleware::map_request_with_state(
            oauth_service.clone(),
            auth,
        ))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found_handler)
        .with_state(oauth_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010")
        .await
        .expect("failed to bind TcpListener");

    axum::serve(listener, app).await.unwrap();
}

pub async fn home_handler() -> Html<String> {
    Html(String::from("Hello"))
}

pub async fn protected_handler(user: User) -> Html<String> {
    Html(format!("Hello {}, here is the s3cret", user.login))
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(String::from("not found")))
}
