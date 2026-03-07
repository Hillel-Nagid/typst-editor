use crate::theme::Theme;
use editor_core::ApplicationState;
use iced::widget::{column, container, text};
use iced::{Background, Border, Element, Length, Padding};
use parking_lot::RwLock;
use std::sync::Arc;

pub fn sidebar_view(
    theme: &Theme,
    state: &Arc<RwLock<ApplicationState>>,
) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.sidebar);
    let fg_color = theme.parse_color(&theme.foreground.sidebar);
    let border_color = theme.parse_color(&theme.ui.border);

    let root_label = state
        .read()
        .get_active_workspace()
        .and_then(|ws| ws.read().root.clone())
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "No folder open".to_string());

    let content = column![
        text("Explorer").size(14),
        text("untitled.typ").size(13),
        text(root_label).size(12),
    ]
    .spacing(8);

    container(content)
        .width(Length::Fixed(240.0))
        .height(Length::Fill)
        .padding(Padding::from([12.0, 10.0]))
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
