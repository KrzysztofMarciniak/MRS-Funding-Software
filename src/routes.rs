use axum::{
    extract::{Path, Request},
    middleware::{from_fn, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken, Key};
use hyper::StatusCode;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

use crate::controllers::{
    about::{
        about, about_all, about_create, about_delete, about_details, about_insert_created,
        about_update,
    },
    campaigns::{
        campaign_details, create_campaign, delete_campaign, edit_campaign, edit_campaign_page, list_all_campaigns, list_campaigns, new_campaign_page, update_amount_page, update_campaign, update_campaign_amount
    },
    contact::{
        contact, contact_all, contact_create, contact_delete, contact_details,
        contact_insert_created, contact_update,
    },
    dashboard::dashboard,
    fourofour::fourofour,
    home::home,
    login::{login_page, login_post},
    logout::logout,
};

pub struct Router {
    session_store: MemoryStore,
    csrf_key: Key,
}

impl Router {
    pub fn new() -> Self {
        Self {
            session_store: MemoryStore::default(),
            csrf_key: Key::generate(),
        }
    }

    pub fn create_router(&self) -> axum::Router {
        let session_layer = SessionManagerLayer::new(self.session_store.clone())
            .with_secure(true)
            .with_http_only(true);

        let csrf_config = CsrfConfig::default()
            .with_key(Some(self.csrf_key.clone()))
            .with_cookie_name("csrf-token");
        let csrf_layer = CsrfLayer::new(csrf_config);

        axum::Router::new()
            .route("/", get(|session: Session| async move { home(&session).await }))
            .route("/about", get(|session: Session| async move { about(&session).await }))
            .route(
                "/login",
                get(|csrf_token: CsrfToken, session: Session| async move { login_page(csrf_token, &session).await })
                    .layer(from_fn(Self::already_logged_in))
                    .post(login_post),
            )
            .route("/logout", get(logout))
            .route(
                "/protected/about/new",
                get(|csrf_token: CsrfToken, session: Session| async move { about_create(csrf_token, &session).await })
                    .post(about_insert_created)
                    .layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/about/all",
                get(|session: Session| async move { about_all(&session).await }).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/about/:id",
                get(|path: axum::extract::Path<i64>, csrf_token: CsrfToken, session: Session| async move {
                    about_details(path, csrf_token, &session).await
                }).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/about/:id/update",
                post(about_update).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/about/:id/delete",
                post(about_delete).layer(from_fn(Self::require_auth)),
            )
            .nest(
                "/protected",
                axum::Router::new()
                    .route("/dashboard", get(|session: Session| async move { dashboard(session).await }))
                    .layer(from_fn(Self::require_auth)),
            )
            .route("/contact", get(|session: Session| async move { contact(&session).await }))
            .route(
                "/protected/contact/new",
                get(|csrf_token: CsrfToken, session: Session| async move { contact_create(csrf_token, &session).await })
                    .post(contact_insert_created)
                    .layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/contact/all",
                get(|session: Session| async move { contact_all(&session).await }).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/contact/:id",
                get(|path: axum::extract::Path<i64>, csrf_token: CsrfToken, session: Session| async move {
                    contact_details(path, csrf_token, &session).await
                }).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/contact/:id/update",
                post(contact_update).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/protected/campaigns/:id/edit",
                get(|path: Path<i64>, csrf_token: CsrfToken, session: Session| async move {
                    edit_campaign_page(path, csrf_token, &session).await
                })
                .post(edit_campaign)
                .layer(from_fn(Self::require_auth)),
            )

            .route("/protected/campaigns", get(|session: Session| async move { list_all_campaigns(&session).await }).layer(from_fn(Self::require_auth)))
            .route(
                "/protected/campaigns/new",
                get(|csrf_token: CsrfToken, session: Session| async move { new_campaign_page(csrf_token, &session).await })
                    .post(create_campaign)
                    .layer(from_fn(Self::require_auth)),
            )            
            .route(
                "/protected/campaigns/:id/delete",
                post(delete_campaign).layer(from_fn(Self::require_auth)),
            )
            .route(
                "/campaigns/:id",
                get(|path: axum::extract::Path<i64>, session: Session| async move {
                    campaign_details(path, &session).await
                }),
            )
            .route(
                "/protected/campaigns/:id/amount",
                post(update_campaign_amount)
                .get(|path: Path<i64>, csrf_token: CsrfToken, session: Session| async move {
                    update_amount_page(path, csrf_token, &session).await
                })
                .layer(from_fn(Self::require_auth)),
            )
            

            .fallback(get(|session: Session| async move { fourofour(&session).await }))
            .layer(session_layer)
            .layer(csrf_layer)
    }

    async fn require_auth(
        session: Session,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        match session.get::<String>("user_id").await {
            Ok(Some(_)) => Ok(next.run(request).await),
            _ => Ok(Redirect::to("/login").into_response()),
        }
    }
    async fn already_logged_in(
        session: Session,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        match session.get::<String>("user_id").await {
            Ok(Some(_)) if request.uri().path() == "/login" => {
                Ok(Redirect::to("/protected/dashboard").into_response())
            }
            Ok(Some(_)) => Ok(next.run(request).await),
            _ => Ok(next.run(request).await),
        }
    }
}
