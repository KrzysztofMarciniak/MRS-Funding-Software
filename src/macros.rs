#[macro_export]
macro_rules! render_page_or_error {
    ($page:expr, $page_name:expr) => {
        $page.render().await.unwrap_or_else(|_| Html(String::from(concat!("Error loading ", $page_name))))
    };
}
