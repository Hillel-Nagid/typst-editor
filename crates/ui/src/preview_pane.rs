use crate::theme::Theme;
use iced::widget::{column, container, text};
use iced::{Background, Border, Element, Length, Padding};

pub fn preview_view(theme: &Theme) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.preview);
    let fg_color = theme.parse_color(&theme.foreground.preview);
    let border_color = theme.parse_color(&theme.ui.border);

    let content = column![text("Preview").size(24), text("PDF preview will appear here").size(13)]
        .spacing(6);

    container(content)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(Padding::from([16.0, 16.0]))
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            text_color: Some(fg_color),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
