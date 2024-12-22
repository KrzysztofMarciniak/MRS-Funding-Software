use axum::response::Redirect;
use tower_sessions::Session;

pub async fn logout(session: Session) -> Redirect {
    session.clear().await;
    Redirect::to("/login")
}
