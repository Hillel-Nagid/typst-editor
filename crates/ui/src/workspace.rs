use crate::app::Message;
use crate::components::status_bar::status_bar_view;
use crate::console::console_view;
use crate::editor::editor_view;
use crate::navbar::navbar_view;
use crate::preview_pane::preview_view;
use crate::sidebar::sidebar_view;
use crate::theme::Theme;
use editor_core::ApplicationState;
use iced::widget::{column, container, row};
use iced::{Background, Element, Length};
use parking_lot::RwLock;
use std::sync::Arc;

pub fn main_window_view(
    state: &Arc<RwLock<ApplicationState>>,
    theme: &Arc<RwLock<Theme>>,
) -> Element<'static, Message> {
    let theme = theme.read().clone();
    let bg_color = theme.parse_color(&theme.background.editor);

    let (sidebar_visible, preview_visible, console_visible) = state
        .read()
        .get_active_workspace()
        .map(|ws| {
            let ws = ws.read();
            (ws.sidebar_visible, ws.preview_visible, ws.console_visible)
        })
        .unwrap_or((true, true, false));

    let mut center_row = row![];
    if sidebar_visible {
        center_row = center_row.push(sidebar_view(&theme, state));
    }

    let mut editor_row = row![editor_view(&theme, state)].height(Length::Fill);
    if preview_visible {
        editor_row = editor_row.push(preview_view(&theme));
    }

    let mut right_column = column![editor_row].height(Length::Fill);
    if console_visible {
        right_column = right_column.push(console_view(&theme));
    }

    center_row = center_row.push(container(right_column).width(Length::Fill));

    let root = column![
        navbar_view(&theme),
        container(center_row).height(Length::Fill),
        status_bar_view(&theme),
    ]
    .height(Length::Fill)
    .width(Length::Fill);

    container(root)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            ..Default::default()
        })
        .into()
}
