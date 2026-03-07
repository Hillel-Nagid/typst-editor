use crate::theme::Theme;
use iced::widget::{container, vertical_space};
use iced::{Background, Border, Element, Length};

pub fn scrollbar_view(theme: &Theme) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.gutter);
    let thumb_color = theme.parse_color(&theme.ui.border);

    let thumb = container(vertical_space())
        .height(Length::Fixed(80.0))
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(thumb_color)),
            border: Border {
                color: thumb_color,
                width: 1.0,
                radius: 2.0.into(),
            },
            ..Default::default()
        });

    container(thumb)
        .width(Length::Fixed(10.0))
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            ..Default::default()
        })
        .into()
}
