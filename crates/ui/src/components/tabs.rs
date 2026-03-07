use crate::theme::Theme;
use iced::widget::{button, row, text};
use iced::{Background, Border, Color, Element, Length, Padding};

pub struct Tab {
    pub id: String,
    pub label: String,
    pub is_dirty: bool,
    pub is_active: bool,
    pub closeable: bool,
}

pub fn tabs_view(
    theme: &Theme,
    tabs: Vec<Tab>,
    on_select: impl Fn(String) -> crate::app::Message + Copy + 'static,
) -> Element<'static, crate::app::Message> {
    let border_color = theme.parse_color(&theme.ui.border);
    let mut tabs_row = row![]
        .width(Length::Fill)
        .spacing(4)
        .padding(Padding::from([4.0, 6.0]));

    for tab in tabs {
        let label = if tab.is_dirty {
            format!("{} ●", tab.label)
        } else {
            tab.label
        };
        let bg = if tab.is_active {
            theme.parse_color(&theme.background.editor)
        } else {
            theme.parse_color(&theme.background.panel)
        };
        let id = tab.id;

        let button = button(text(label).size(12))
            .style(move |_, _| iced::widget::button::Style {
                background: Some(Background::Color(bg)),
                text_color: Color::WHITE,
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .on_press(on_select(id));

        tabs_row = tabs_row.push(button);
    }

    tabs_row.into()
}
