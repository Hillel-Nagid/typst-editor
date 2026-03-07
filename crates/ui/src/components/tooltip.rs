use crate::theme::Theme;
use iced::widget::{container, text};
use iced::{Background, Border, Element, Padding};

pub fn tooltip_view(
    theme: &Theme,
    content: impl Into<String>,
    visible: bool,
) -> Element<'static, crate::app::Message> {
    if !visible {
        return container(text("")).into();
    }

    let bg_color = theme.parse_color(&theme.background.panel);
    let fg_color = theme.parse_color(&theme.foreground.panel);
    let border_color = theme.parse_color(&theme.ui.border);

    container(text(content.into()).size(12))
        .padding(Padding::from([4.0, 6.0]))
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
