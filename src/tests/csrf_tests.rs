use crate::routes::Router;
use std::env;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
#[tokio::test]
async fn test_csrf_protection_blocks_unauthorized_post() {
    let router = Router::new();
    let app = router.create_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_csrf_token_present_in_form() {
    let router = Router::new();
    let app = router.create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("csrf_token"));
}
#[tokio::test]
async fn test_login_with_invalid_csrf_token() {
    env::set_var("DATABASE_URL", ":memory:");
    env::set_var("ADMIN_USERNAME", "admin");
    env::set_var("ADMIN_PASSWORD", "admin");
    let conn = rusqlite::Connection::open(":memory:").unwrap();
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        ["admin", "admin"],
    )
    .unwrap();

    let router = Router::new();
    let app = router.create_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(
                    "username=admin&password=admin&csrf_token=totally_invalid_token",
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("location").unwrap(), "/login");
}
