use crate::controllers::page::Page;
use crate::render_page_or_error;
use axum::http::StatusCode;
use axum::response::Html;

pub async fn fourofour(session: &tower_sessions::Session) -> Html<String> {
    let mut page = Page::new("404 - Page Not Found", session)
        .with_status(StatusCode::NOT_FOUND)
        .with_meta_description("Page not found");

    let content = r#"
        <section class="error-page">
            <h1>404 - Page Not Found 🔍</h1>
            <div class="error-content">
                <div class="helpful-links">
                    <ul>
                        <li><a href="/">Return to Homepage</a></li>
                    </ul>
                </div>
            </div>
        </section>
    "#;

    page.set_content(content.to_string());
    render_page_or_error!(page, "404 page")
}
