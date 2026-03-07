use crate::theme::Theme;
use iced::widget::{column, container, row, text};
use iced::{Background, Border, Element, Length, Padding};

pub fn console_view(theme: &Theme) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.panel);
    let fg_color = theme.parse_color(&theme.foreground.panel);
    let border_color = theme.parse_color(&theme.ui.border);

    let content = column![
        row![text("Problems"), text("Output"), text("Terminal")]
            .spacing(16)
            .padding(Padding::from([0.0, 4.0])),
        text("No problems detected").size(13),
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(180.0))
        .padding(Padding::from([8.0, 10.0]))
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
