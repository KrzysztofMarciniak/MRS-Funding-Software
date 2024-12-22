use axum::{
    http::StatusCode,
    response::Html,
};
use axum_csrf::CsrfToken;
use tower_sessions::Session;

use crate::views::layout::render_layout;

pub struct Page<'a> {
    title: &'a str,
    content: String,
    session: &'a Session,
    csrf_token: Option<String>,
    status: StatusCode,
    meta_description: Option<String>,
}

impl<'a> Page<'a> {
    pub fn new(title: &'a str, session: &'a Session) -> Self {
        Self {
            title,
            content: String::new(),
            session,
            csrf_token: None,
            status: StatusCode::OK,
            meta_description: None,
        }
    }

    pub fn with_csrf_token(mut self, csrf_token: CsrfToken) -> Self {
        self.csrf_token = csrf_token.authenticity_token().ok();
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_meta_description(mut self, description: &str) -> Self {
        self.meta_description = Some(description.to_string());
        self
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub async fn render(self) -> Result<Html<String>, StatusCode> {
        Ok(render_layout(&self.content, self.session).await)
    }
    pub fn get_csrf_token(&self) -> Option<&String> {
        self.csrf_token.as_ref()
    }
}
