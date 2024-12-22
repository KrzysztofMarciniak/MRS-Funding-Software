use crate::{db, render_page_or_error};
use axum::{
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use axum_csrf::CsrfToken;
use serde::Deserialize;
use tower_sessions::Session;
use crate::controllers::page::Page;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_page(csrf_token: CsrfToken, session: &Session) -> Html<String> {
    let mut page = Page::new("Login", session)
        .with_csrf_token(csrf_token)
        .with_meta_description("Login to access the admin dashboard");

    let content = format!(
        r#"
        <section class="login-form">
            <h2>Login</h2>
            <form method="POST" action="/login">
                <input type="hidden" name="csrf_token" value="{}">
                <div class="form-group">
                    <input type="text" name="username" placeholder="Username" required>
                </div>
                <div class="form-group">
                    <input type="password" name="password" placeholder="Password" required>
                </div>
                <button type="submit">Login</button>
            </form>
        </section>
        "#,
        page.get_csrf_token().unwrap_or(&String::new())
    );

    page.set_content(content);
    render_page_or_error!(page, "login page")
}

pub async fn login_post(
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, (StatusCode, String)> {
    match db::get_connection() {
        Ok(conn) => {
            let result = conn.query_row(
                "SELECT id FROM users WHERE username = ? AND password = ?",
                [&form.username, &form.password],
                |row| row.get::<_, i64>(0),
            );

            match result {
                Ok(_) => {
                    session
                        .insert("user_id", form.username)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    Ok(Redirect::to("/protected/dashboard"))
                }
                Err(_) => Ok(Redirect::to("/login"))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}