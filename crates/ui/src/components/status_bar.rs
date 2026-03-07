use crate::theme::Theme;
use iced::widget::{container, horizontal_space, row, text};
use iced::{Background, Border, Element, Length, Padding};

pub fn status_bar_view(theme: &Theme) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.panel);
    let fg_color = theme.parse_color(&theme.foreground.panel);
    let border_color = theme.parse_color(&theme.ui.border);

    let left = row![text("Typst"), text("Line 1, Col 1")]
        .spacing(12)
        .padding(Padding::from([0.0, 4.0]));
    let right = row![text("UTF-8"), text("LF")]
        .spacing(12)
        .padding(Padding::from([0.0, 4.0]));

    container(row![left, horizontal_space(), right].width(Length::Fill).spacing(12))
    .width(Length::Fill)
    .padding(Padding::from([6.0, 10.0]))
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
