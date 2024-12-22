use crate::routes::Router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;
use tower_sessions::{MemoryStore, Session};
#[tokio::test]
async fn test_logout_with_active_session() {
    let session_store = MemoryStore::default();
    let router = Router::new();
    let app = router.create_router();

    let session = Session::new(None, Arc::new(session_store.clone()), None);
    session.insert("user_id", "test_user").await.unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/logout")
                .extension(session)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("location").unwrap(), "/login");
}
