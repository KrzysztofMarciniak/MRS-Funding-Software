pub fn render_head(title: &str, theme_css: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}</title>
            <style>
                {}
            </style>
        </head>
        <body>
        "#,
        title, theme_css
    )
}
