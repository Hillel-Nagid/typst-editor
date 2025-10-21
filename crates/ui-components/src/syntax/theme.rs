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
        todo!("Implement default light theme")
    }

    pub fn default_dark() -> Self {
        todo!("Implement default dark theme")
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
        todo!("Implement theme loading from file")
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
