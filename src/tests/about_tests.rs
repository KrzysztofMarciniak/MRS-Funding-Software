use crate::routes::Router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::env;
use tower::ServiceExt;

#[tokio::test]
async fn test_about_page_display() {
    env::set_var("DATABASE_URL", ":memory:");
    let router = Router::new();
    let app = router.create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/about")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("About Us"));
}
