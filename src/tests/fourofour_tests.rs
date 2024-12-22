use crate::routes::Router;
use axum::body::to_bytes;
use axum::{body::Body, http::Request};
use tower::ServiceExt;
#[tokio::test]
async fn test_404_response() {
    let router = Router::new();
    let app = router.create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/non-existent-path")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_string.contains("404"));
}
