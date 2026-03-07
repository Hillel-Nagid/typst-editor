use crate::theme::Theme;
use iced::widget::{container, text_input};
use iced::{Background, Border, Element, Length};

pub fn input_view(
    theme: &Theme,
    value: &str,
    placeholder: &str,
    on_change: impl Fn(String) -> crate::app::Message + 'static,
) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.ui.input_background);
    let border_color = theme.parse_color(&theme.ui.input_border);
    let fg_color = theme.parse_color(&theme.foreground.editor);

    container(text_input(placeholder, value).on_input(on_change))
        .width(Length::Fill)
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
