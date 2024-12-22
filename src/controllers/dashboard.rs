use crate::{controllers::page::Page, render_page_or_error};
use axum::response::Html;
use tower_sessions::Session;


pub async fn dashboard(session: Session) -> Html<String> {
    let username = session
        .get::<String>("user_id")
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let mut page = Page::new("Dashboard", &session)
        .with_meta_description("Admin Dashboard - Manage your crowdfunding platform");

    let content = format!(
        r#"
        <section class="dashboard">
            <h1>Dashboard</h1>
            <p class="user-info">Logged in as: <strong>{}</strong></p>
            <div class="dashboard-links">
                <a href="/protected/about/all" class="button">Open About Me Options</a>
                <a href="/protected/contact/all" class="button">Open Contact Options</a>
                <a href="/protected/campaigns" class="button">Open Campaign Options</a>
            </div>
        </section>
        "#,
        username,
    );

    page.set_content(content);
    render_page_or_error!(page, "dashboard")
}
