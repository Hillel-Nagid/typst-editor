//! Theme system for syntax highlighting and UI colors
//!
//! Phase 3.3: Syntax Highlighting

use palette::Srgb;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub colors: ColorScheme,
    pub typography: Typography,
    pub spacing: Spacing,
}

impl Theme {
    pub fn default_light() -> Self {
        Self {
            name: "Light".to_string(),
            variant: ThemeVariant::Light,
            colors: ColorScheme {
                background: Srgb::new(1.0, 1.0, 1.0),
                foreground: Srgb::new(0.0, 0.0, 0.0),
                border: Srgb::new(0.8, 0.8, 0.8),
                selection: Srgb::new(0.0, 0.5, 0.9),
                cursor: Srgb::new(0.0, 0.0, 0.0),
                current_line: Srgb::new(0.95, 0.95, 0.95),
                keyword: Srgb::new(0.7, 0.1, 0.3),
                function: Srgb::new(0.1, 0.3, 0.8),
                variable: Srgb::new(0.0, 0.0, 0.0),
                constant: Srgb::new(0.6, 0.2, 0.5),
                string: Srgb::new(0.2, 0.6, 0.3),
                comment: Srgb::new(0.4, 0.4, 0.4),
                type_name: Srgb::new(0.4, 0.2, 0.7),
                operator: Srgb::new(0.5, 0.5, 0.5),
                error: Srgb::new(1.0, 0.0, 0.0),
                warning: Srgb::new(1.0, 0.6, 0.0),
                info: Srgb::new(0.0, 0.5, 0.9),
                hint: Srgb::new(0.5, 0.5, 0.5),
                button_background: Srgb::new(0.9, 0.9, 0.9),
                button_hover: Srgb::new(0.8, 0.8, 0.8),
                input_background: Srgb::new(1.0, 1.0, 1.0),
                panel_background: Srgb::new(0.95, 0.95, 0.95),
                sidebar_background: Srgb::new(0.92, 0.92, 0.92),
                statusbar_background: Srgb::new(0.88, 0.88, 0.88),
            },
            typography: Typography {
                editor_font: "Consolas".to_string(),
                editor_size: 14.0,
                ui_font: "Segoe UI".to_string(),
                ui_size: 12.0,
                line_height: 1.5,
            },
            spacing: Spacing {
                gutter_width: 50.0,
                line_padding: 2.0,
                panel_padding: 8.0,
            },
        }
    }

    pub fn default_dark() -> Self {
        Self {
            name: "Dark".to_string(),
            variant: ThemeVariant::Dark,
            colors: ColorScheme {
                background: Srgb::new(0.1, 0.1, 0.1),
                foreground: Srgb::new(0.9, 0.9, 0.9),
                border: Srgb::new(0.3, 0.3, 0.3),
                selection: Srgb::new(0.2, 0.5, 0.8),
                cursor: Srgb::new(1.0, 1.0, 1.0),
                current_line: Srgb::new(0.15, 0.15, 0.15),
                keyword: Srgb::new(0.9, 0.4, 0.6),
                function: Srgb::new(0.4, 0.6, 0.9),
                variable: Srgb::new(0.9, 0.9, 0.9),
                constant: Srgb::new(0.8, 0.4, 0.7),
                string: Srgb::new(0.4, 0.8, 0.5),
                comment: Srgb::new(0.5, 0.5, 0.5),
                type_name: Srgb::new(0.6, 0.4, 0.8),
                operator: Srgb::new(0.7, 0.7, 0.7),
                error: Srgb::new(1.0, 0.3, 0.3),
                warning: Srgb::new(1.0, 0.7, 0.3),
                info: Srgb::new(0.3, 0.7, 1.0),
                hint: Srgb::new(0.7, 0.7, 0.7),
                button_background: Srgb::new(0.2, 0.2, 0.2),
                button_hover: Srgb::new(0.3, 0.3, 0.3),
                input_background: Srgb::new(0.15, 0.15, 0.15),
                panel_background: Srgb::new(0.12, 0.12, 0.12),
                sidebar_background: Srgb::new(0.14, 0.14, 0.14),
                statusbar_background: Srgb::new(0.16, 0.16, 0.16),
            },
            typography: Typography {
                editor_font: "Consolas".to_string(),
                editor_size: 14.0,
                ui_font: "Segoe UI".to_string(),
                ui_size: 12.0,
                line_height: 1.5,
            },
            spacing: Spacing {
                gutter_width: 50.0,
                line_padding: 2.0,
                panel_padding: 8.0,
            },
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_light()
    }
}

/// Theme variant (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeVariant {
    Light,
    Dark,
}

/// Color scheme for the theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // UI colors
    #[serde(with = "serde_srgb")]
    pub background: Srgb,
    #[serde(with = "serde_srgb")]
    pub foreground: Srgb,
    #[serde(with = "serde_srgb")]
    pub border: Srgb,
    #[serde(with = "serde_srgb")]
    pub selection: Srgb,
    #[serde(with = "serde_srgb")]
    pub cursor: Srgb,
    #[serde(with = "serde_srgb")]
    pub current_line: Srgb,

    // Syntax colors
    #[serde(with = "serde_srgb")]
    pub keyword: Srgb,
    #[serde(with = "serde_srgb")]
    pub function: Srgb,
    #[serde(with = "serde_srgb")]
    pub variable: Srgb,
    #[serde(with = "serde_srgb")]
    pub constant: Srgb,
    #[serde(with = "serde_srgb")]
    pub string: Srgb,
    #[serde(with = "serde_srgb")]
    pub comment: Srgb,
    #[serde(with = "serde_srgb")]
    pub type_name: Srgb,
    #[serde(with = "serde_srgb")]
    pub operator: Srgb,

    // Semantic colors
    #[serde(with = "serde_srgb")]
    pub error: Srgb,
    #[serde(with = "serde_srgb")]
    pub warning: Srgb,
    #[serde(with = "serde_srgb")]
    pub info: Srgb,
    #[serde(with = "serde_srgb")]
    pub hint: Srgb,

    // UI element colors
    #[serde(with = "serde_srgb")]
    pub button_background: Srgb,
    #[serde(with = "serde_srgb")]
    pub button_hover: Srgb,
    #[serde(with = "serde_srgb")]
    pub input_background: Srgb,
    #[serde(with = "serde_srgb")]
    pub panel_background: Srgb,
    #[serde(with = "serde_srgb")]
    pub sidebar_background: Srgb,
    #[serde(with = "serde_srgb")]
    pub statusbar_background: Srgb,
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub editor_font: String,
    pub editor_size: f32,
    pub ui_font: String,
    pub ui_size: f32,
    pub line_height: f32,
}

/// Spacing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub gutter_width: f32,
    pub line_padding: f32,
    pub panel_padding: f32,
}

/// Custom serde module for Srgb serialization
mod serde_srgb {
    use palette::Srgb;
    use serde::{ Deserialize, Deserializer, Serialize, Serializer };

    #[derive(Serialize, Deserialize)]
    struct SrgbHelper {
        r: f32,
        g: f32,
        b: f32,
    }

    pub fn serialize<S>(color: &Srgb, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let helper = SrgbHelper {
            r: color.red,
            g: color.green,
            b: color.blue,
        };
        helper.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Srgb, D::Error> where D: Deserializer<'de> {
        let helper = SrgbHelper::deserialize(deserializer)?;
        Ok(Srgb::new(helper.r, helper.g, helper.b))
    }
}

/// Theme manager for loading and managing themes
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    active_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("light".to_string(), Theme::default_light());
        themes.insert("dark".to_string(), Theme::default_dark());

        Self {
            themes,
            active_theme: "light".to_string(),
        }
    }

    pub fn get_active_theme(&self) -> &Theme {
        self.themes.get(&self.active_theme).unwrap()
    }

    pub fn set_active_theme(&mut self, name: String) {
        if self.themes.contains_key(&name) {
            self.active_theme = name;
        }
    }

    pub fn load_theme(&mut self, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement theme loading from file
        Ok(())
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
