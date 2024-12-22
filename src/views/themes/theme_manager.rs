use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub struct ThemeManager {
    themes: HashMap<String, String>,
    active_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            active_theme: "default".to_string(),
        };
        manager.load_themes();
        manager.set_active_theme_from_env();
        manager
    }

    fn load_themes(&mut self) {
        let themes_dir = Path::new("src/views/themes");
        if let Ok(entries) = fs::read_dir(themes_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".css") {
                        let theme_name = filename.trim_end_matches(".css").to_string();
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            self.themes.insert(theme_name, content);
                        }
                    }
                }
            }
        }
    }

    fn set_active_theme_from_env(&mut self) {
        let env_theme = env::var("ACTIVE_THEME").unwrap_or_else(|_| "default".to_string());
        if self.themes.contains_key(&env_theme) {
            self.active_theme = env_theme;
        } else {
            println!(
                "Error: Theme '{}' not found. Defaulting to 'default'.",
                env_theme
            );
            self.active_theme = "default".to_string();
        }
    }

    pub fn get_active_theme_css(&self) -> String {
        self.themes
            .get(&self.active_theme)
            .cloned()
            .unwrap_or_else(|| self.themes.get("default").cloned().unwrap_or_default())
    }
}
