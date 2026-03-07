use crate::theme::Theme;
use iced::widget::{button, text};
use iced::{Background, Border, Color, Element, Padding};

#[derive(Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

pub struct Button {
    label: String,
    variant: ButtonVariant,
    on_click: Option<crate::app::Message>,
}
impl Button {
    pub fn new(label: impl Into<String>, variant: ButtonVariant) -> Self {
        Self {
            label: label.into(),
            variant,
            on_click: None,
        }
    }

    pub fn on_click(mut self, message: crate::app::Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn view(self, theme: &Theme) -> Element<'static, crate::app::Message> {
        let (bg, fg) = match self.variant {
            ButtonVariant::Primary => (
                theme.parse_color(&theme.ui.button_background),
                theme.parse_color(&theme.foreground.editor),
            ),
            ButtonVariant::Secondary => (
                theme.parse_color(&theme.ui.input_background),
                theme.parse_color(&theme.foreground.editor),
            ),
            ButtonVariant::Danger => (
                theme.parse_color(&theme.semantic.error),
                Color::WHITE,
            ),
            ButtonVariant::Ghost => (Color::TRANSPARENT, theme.parse_color(&theme.foreground.editor)),
        };

        let mut b = button(text(self.label).size(12))
            .style(move |_, _| iced::widget::button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding(Padding::from([4.0, 8.0]));

        if let Some(msg) = self.on_click {
            b = b.on_press(msg);
        }

        b.into()
    }
}
