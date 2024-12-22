pub struct Footer {}

impl Footer {
    pub fn render() -> String {
        r#"
        <footer>
            <p class="footer-ad"><a href="https://github.com/KrzysztofMarciniak/MRS-Funding-Software">Proudly using MRS Funding Software.</a></p>
        </footer>
        </body>
        "#
        .to_string()
    }
}
