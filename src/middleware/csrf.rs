use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_csrf::{CsrfToken, Key};
use http::StatusCode;
use tower_sessions::Session;

pub struct CsrfVerifier;

#[async_trait]
impl<S> FromRequestParts<S> for CsrfVerifier
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = parts
            .extract::<Session>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

        let csrf_token = parts
            .extract::<CsrfToken>()
            .await
            .map_err(|_| StatusCode::FORBIDDEN.into_response())?;

        Ok(CsrfVerifier)
    }
}
