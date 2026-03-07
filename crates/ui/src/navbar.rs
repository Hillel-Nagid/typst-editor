use crate::app::Message;
use crate::theme::Theme;
use iced::alignment::Vertical;
use iced::widget::{ button, container, row, text };
use iced::{ Alignment, Background, Border, Color, Element, Length, Padding };

pub fn navbar_view(theme: &Theme) -> Element<'static, Message> {
    let bg_color = theme.parse_color(&theme.background.titlebar);
    let fg_color = theme.parse_color(&theme.foreground.titlebar);
    let border_color = theme.parse_color(&theme.ui.border);
    let button_bg = theme.parse_color(&theme.ui.button_background);
    let button_fg = theme.parse_color(&theme.foreground.editor);

    let menu_button = |label: &'static str, msg: Message| {
        button(text(label).size(12))
            .style(move |_, _| iced::widget::button::Style {
                background: Some(Background::Color(button_bg)),
                text_color: button_fg,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: (4.0).into(),
                },
                ..Default::default()
            })
            .padding(Padding::from([4.0, 8.0]))
            .on_press(msg)
    };

    let content = row![
        row![
            text("Typst Studio").size(18),
            menu_button("File", Message::ToggleSidebar),
            menu_button("View", Message::TogglePreview),
            menu_button("Tools", Message::ToggleConsole),
            menu_button("Undo", Message::UndoEdit),
            menu_button("Redo", Message::RedoEdit)
        ]
            .spacing(12)
            .align_y(Vertical::Center),
        row![text("Search"), text("Settings")].spacing(12).align_y(Vertical::Center)
    ]
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .spacing(16);

    container(content)
        .width(Length::Fill)
        .padding(Padding::from([8.0, 12.0]))
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            text_color: Some(fg_color),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: (0.0).into(),
            },
            ..Default::default()
        })
        .align_x(Alignment::Start)
        .into()
}
