use crate::theme::Theme;
use iced::widget::{column, container, text};
use iced::{Background, Border, Element, Length, Padding};
use std::sync::Arc;

pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub action: Option<Arc<dyn Fn() -> crate::app::Message + Send + Sync + 'static>>,
    pub is_separator: bool,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            enabled: true,
            action: None,
            is_separator: false,
        }
    }

    pub fn separator() -> Self {
        Self {
            label: String::new(),
            shortcut: None,
            enabled: true,
            action: None,
            is_separator: true,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn on_select<F>(mut self, handler: F) -> Self
    where
        F: Fn() -> crate::app::Message + Send + Sync + 'static,
    {
        self.action = Some(Arc::new(handler));
        self
    }
}

pub fn context_menu_view(
    theme: &Theme,
    items: &[MenuItem],
    visible: bool,
) -> Element<'static, crate::app::Message> {
    if !visible {
        return container(text("")).into();
    }

    let bg_color = theme.parse_color(&theme.background.panel);
    let fg_color = theme.parse_color(&theme.foreground.panel);
    let border_color = theme.parse_color(&theme.ui.border);

    let mut content = column![];
    for item in items {
        if item.is_separator {
            content = content.push(text("----------------"));
        } else {
            let label = if let Some(shortcut) = &item.shortcut {
                format!("{}    {}", item.label, shortcut)
            } else {
                item.label.clone()
            };
            content = content.push(text(label).size(12));
        }
    }

    container(content.spacing(6))
        .width(Length::Shrink)
        .padding(Padding::from([8.0, 8.0]))
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            text_color: Some(fg_color),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
}
