use axum::{
    extract::{Request, State},
    middleware, RequestExt, Router,
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use axum_github_oauth::{AuthAction, GithubOauthService, User};

/// Middleware to check if the user is authorized.
async fn auth(
    State(state): State<GithubOauthService>,
    mut request: Request,
) -> Result<Request, AuthAction> {
    match request.uri().path() {
        path if state.is_public(path) => Ok(request),
        _ => request
            .extract_parts_with_state::<User, GithubOauthService>(&state)
            .await
            .map(|_user| request),
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

pub async fn protected_handler() -> Html<String> {
    Html(String::from("s3cret"))
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(String::from("not found")))
}
