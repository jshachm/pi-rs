//! Theme parser and definitions

use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub name: String,
    #[serde(default)]
    pub colors: ThemeColors,
    #[serde(default)]
    pub styles: ThemeStyles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeColors {
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub secondary: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub foreground: String,
    #[serde(default)]
    pub success: String,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub warning: String,
    #[serde(default)]
    pub info: String,
    #[serde(default)]
    pub user_message: String,
    #[serde(default)]
    pub assistant_message: String,
    #[serde(default)]
    pub system_message: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "cyan".to_string(),
            secondary: "magenta".to_string(),
            accent: "yellow".to_string(),
            background: "black".to_string(),
            foreground: "white".to_string(),
            success: "green".to_string(),
            error: "red".to_string(),
            warning: "yellow".to_string(),
            info: "blue".to_string(),
            user_message: "cyan".to_string(),
            assistant_message: "green".to_string(),
            system_message: "yellow".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ThemeStyles {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            name: "default".to_string(),
            colors: ThemeColors::default(),
            styles: ThemeStyles::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            colors: ThemeColors {
                primary: "cyan".to_string(),
                secondary: "magenta".to_string(),
                accent: "yellow".to_string(),
                background: "#1e1e2e".to_string(),
                foreground: "#cdd6f4".to_string(),
                success: "green".to_string(),
                error: "red".to_string(),
                warning: "yellow".to_string(),
                info: "blue".to_string(),
                user_message: "cyan".to_string(),
                assistant_message: "green".to_string(),
                system_message: "yellow".to_string(),
            },
            styles: ThemeStyles::default(),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            colors: ThemeColors {
                primary: "blue".to_string(),
                secondary: "magenta".to_string(),
                accent: "orange".to_string(),
                background: "#ffffff".to_string(),
                foreground: "#333333".to_string(),
                success: "green".to_string(),
                error: "red".to_string(),
                warning: "yellow".to_string(),
                info: "blue".to_string(),
                user_message: "blue".to_string(),
                assistant_message: "green".to_string(),
                system_message: "magenta".to_string(),
            },
            styles: ThemeStyles::default(),
        }
    }

    pub fn parse_color(&self, name: &str) -> Color {
        match name.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "reset" => Color::Reset,
            s if s.starts_with('#') => {
                if let Ok(hex) = u32::from_str_radix(&s[1..], 16) {
                    let r = ((hex >> 16) & 0xff) as u8;
                    let g = ((hex >> 8) & 0xff) as u8;
                    let b = (hex & 0xff) as u8;
                    Color::Rgb(r, g, b)
                } else {
                    Color::White
                }
            }
            _ => Color::White,
        }
    }

    pub fn get_color(&self, color_type: &str) -> Color {
        let color_name = match color_type {
            "primary" => &self.colors.primary,
            "secondary" => &self.colors.secondary,
            "accent" => &self.colors.accent,
            "background" => &self.colors.background,
            "foreground" => &self.colors.foreground,
            "success" => &self.colors.success,
            "error" => &self.colors.error,
            "warning" => &self.colors.warning,
            "info" => &self.colors.info,
            "user_message" => &self.colors.user_message,
            "assistant_message" => &self.colors.assistant_message,
            "system_message" => &self.colors.system_message,
            _ => return Color::White,
        };
        self.parse_color(color_name)
    }
}
