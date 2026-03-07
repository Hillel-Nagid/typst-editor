use crate::theme::Theme;
use iced::widget::{container, horizontal_space, vertical_space};
use iced::{Background, Element, Length};

pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub fn splitter_view(
    theme: &Theme,
    direction: SplitDirection,
) -> Element<'static, crate::app::Message> {
    let divider_color = theme.parse_color(&theme.ui.divider);
    match direction {
        SplitDirection::Horizontal => container(horizontal_space())
            .height(Length::Fixed(1.0))
            .width(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(divider_color)),
                ..Default::default()
            })
            .into(),
        SplitDirection::Vertical => container(vertical_space())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(divider_color)),
                ..Default::default()
            })
            .into(),
    }
}
