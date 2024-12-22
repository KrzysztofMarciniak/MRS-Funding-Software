use crate::views::blocks::footer::Footer;
use crate::views::blocks::head::render_head;
use crate::views::blocks::header::Header;
use crate::views::themes::theme_manager::ThemeManager;
use axum::response::Html;
use tower_sessions::Session;

pub struct Layout {
    title: String,
    content: String,
    is_authenticated: bool,
    theme_manager: ThemeManager,
}

impl Layout {
    pub fn new(title: String, content: String, is_authenticated: bool) -> Self {
        let theme_manager = ThemeManager::new();
        Self {
            title,
            content,
            is_authenticated,
            theme_manager,
        }
    }

    pub async fn render(self) -> Html<String> {
        let content = format!(
            r#"
            {}
                {}
                <main>
                    {}
                </main>
                {}
            </body>
            </html>
            "#,
            render_head(&self.title, &self.theme_manager.get_active_theme_css()),
            Header::new(self.title.clone(), self.is_authenticated).render(),
            self.content,
            Footer::render()
        );
        Html(content)
    }
}

pub async fn render_layout(content: &str, session: &Session) -> Html<String> {
    let website_title =
        std::env::var("WEBSITE_TITLE")
        .unwrap_or_else(|_| "MRS-Funding-Software".to_string());

    let is_authenticated = session
        .get::<String>("user_id")
        .await
        .unwrap_or(None)
        .is_some();

    Layout::new(website_title, content.to_string(), is_authenticated)
        .render()
        .await
}
