pub struct Header {
    title: String,
    is_authenticated: bool,
}

impl Header {
    pub fn new(title: String, is_authenticated: bool) -> Self {
        Self {
            title,
            is_authenticated,
        }
    }

    pub fn render(&self) -> String {
        let menu_items = if self.is_authenticated {
            r#"
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
                <li><a href="/protected/dashboard">Dashboard</a></li>
                <li><a href="/logout">Logout</a></li>
            "#
        } else {
            r#"
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
                <li><a href="/login">Login</a></li>
            "#
        };

        format!(
            r#"
            <header>
                <h1>{}</h1>
                <nav>
                    <ul>
                        {}
                    </ul>
                </nav>
            </header>
            "#,
            self.title, menu_items
        )
    }
}
