use crate::theme::Theme;
use editor_core::ApplicationState;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Border, Color, Element, Length, Padding};
use parking_lot::RwLock;
use std::ops::Range;
use std::sync::Arc;

pub fn editor_view(
    theme: &Theme,
    state: &Arc<RwLock<ApplicationState>>,
) -> Element<'static, crate::app::Message> {
    let bg_color = theme.parse_color(&theme.background.editor);
    let fg_color = theme.parse_color(&theme.foreground.editor);
    let gutter_bg = theme.parse_color(&theme.background.gutter);
    let gutter_fg = theme.parse_color(&theme.foreground.gutter);
    let border_color = theme.parse_color(&theme.ui.border);

    let (lines, line_selections, line_cursor_cols, cursor_line) =
        if let Some(workspace) = state.read().get_active_workspace() {
        let workspace = workspace.read();
        if let Some(editor) = workspace.get_active_editor() {
            let editor = editor.read();
            let content = editor.content();
            let cursors = editor.cursors().cursors().to_vec();
            let lines = split_lines_preserving_trailing(&content);
            let line_start_offsets = compute_line_start_offsets(&content);
            let line_ranges = compute_line_content_ranges(&line_start_offsets, content.chars().count());
            let line_selections = compute_line_selections(&lines, &line_start_offsets, &line_ranges, &cursors);
            let line_cursor_cols = compute_line_cursor_cols(&lines, &line_start_offsets, &cursors);
            let (line_idx, _) = offset_to_line_col(&line_start_offsets, cursors[editor.cursors().primary_index()].position());
            (lines, line_selections, line_cursor_cols, line_idx)
        } else {
            (
                vec!["// No document open".to_string()],
                vec![Vec::new()],
                vec![Vec::new()],
                0,
            )
        }
    } else {
        (
            vec!["// No workspace".to_string()],
            vec![Vec::new()],
            vec![Vec::new()],
            0,
        )
    };

    let line_count = lines.len().max(1);

    let gutter = column((1..=line_count).map(|i| text(i.to_string()).size(12).into()))
        .spacing(2)
        .padding(Padding::from([8.0, 8.0]));

    let selection_bg = theme.parse_color(&theme.ui.selection_background);
    let selection_fg = theme.parse_color(&theme.ui.selection_foreground);
    let cursor_color = theme.parse_color(&theme.ui.cursor);

    let line_widgets = lines
        .into_iter()
        .enumerate()
        .fold(column!().spacing(2), |col, (idx, line)| {
            let line_bg = if idx == cursor_line {
                theme.parse_color(&theme.ui.line_highlight)
            } else {
                Color::TRANSPARENT
            };
            let rendered_line = render_line_with_cursors_and_selection(
                line,
                line_selections.get(idx).cloned().unwrap_or_default(),
                line_cursor_cols.get(idx).cloned().unwrap_or_default(),
                selection_bg,
                selection_fg,
                cursor_color,
            );
            col.push(
                container(rendered_line)
                    .padding(Padding::from([0.0, 8.0]))
                    .style(move |_| iced::widget::container::Style {
                        background: Some(Background::Color(line_bg)),
                        ..Default::default()
                    }),
            )
        });
    let editor_text = scrollable(column![
        line_widgets,
        text("Keyboard: type, Enter, Backspace/Delete, arrows, Ctrl/Cmd shortcuts").size(11),
    ]);

    let view = row![
        container(gutter)
            .width(Length::Fixed(50.0))
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(gutter_bg)),
                text_color: Some(gutter_fg),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
        container(editor_text).width(Length::Fill).height(Length::Fill),
    ];

    container(view)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            text_color: Some(fg_color),
            ..Default::default()
        })
        .into()
}

fn split_lines_preserving_trailing(content: &str) -> Vec<String> {
    if content.is_empty() {
        return vec![String::new()];
    }
    content.split('\n').map(|line| line.to_string()).collect()
}

fn compute_line_start_offsets(content: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (idx, ch) in content.chars().enumerate() {
        if ch == '\n' {
            starts.push(idx + 1);
        }
    }
    starts
}

fn compute_line_content_ranges(line_starts: &[usize], content_len: usize) -> Vec<Range<usize>> {
    let mut ranges = Vec::with_capacity(line_starts.len());
    for (idx, start) in line_starts.iter().copied().enumerate() {
        let end = if idx + 1 < line_starts.len() {
            line_starts[idx + 1].saturating_sub(1)
        } else {
            content_len
        };
        ranges.push(start..end.max(start));
    }
    ranges
}

fn line_display_chars(line: &str) -> Vec<char> {
    line.chars().filter(|ch| *ch != '\r').collect()
}

fn display_col_from_global_offset(line: &str, line_start: usize, offset: usize) -> usize {
    let raw_col = offset.saturating_sub(line_start).min(line.chars().count());
    line.chars().take(raw_col).filter(|ch| *ch != '\r').count()
}

fn offset_to_line_col(line_starts: &[usize], offset: usize) -> (usize, usize) {
    let line = line_starts.partition_point(|start| *start <= offset).saturating_sub(1);
    (line, offset.saturating_sub(line_starts[line]))
}

fn compute_line_selections(
    lines: &[String],
    line_starts: &[usize],
    line_ranges: &[Range<usize>],
    cursors: &[editor_core::Cursor],
) -> Vec<Vec<Range<usize>>> {
    let mut all = vec![Vec::new(); lines.len()];
    for cursor in cursors {
        if !cursor.has_selection() {
            continue;
        }
        let selection = cursor.range();
        for line_idx in 0..lines.len() {
            let range = &line_ranges[line_idx];
            let overlap_start = selection.start.max(range.start);
            let overlap_end = selection.end.min(range.end);
            if overlap_start < overlap_end {
                let start_col =
                    display_col_from_global_offset(&lines[line_idx], line_starts[line_idx], overlap_start);
                let end_col =
                    display_col_from_global_offset(&lines[line_idx], line_starts[line_idx], overlap_end);
                if end_col > start_col {
                    all[line_idx].push(start_col..end_col);
                }
            }
        }
    }
    all
}

fn compute_line_cursor_cols(
    lines: &[String],
    line_starts: &[usize],
    cursors: &[editor_core::Cursor],
) -> Vec<Vec<usize>> {
    let mut all = vec![Vec::new(); lines.len()];
    for cursor in cursors {
        let position = cursor.position();
        let (line_idx, _) = offset_to_line_col(line_starts, position);
        let display_col = display_col_from_global_offset(&lines[line_idx], line_starts[line_idx], position);
        all[line_idx].push(display_col);
    }
    all
}

fn render_line_with_cursors_and_selection(
    line: String,
    selections: Vec<Range<usize>>,
    cursor_cols: Vec<usize>,
    selection_bg: Color,
    selection_fg: Color,
    cursor_color: Color,
) -> iced::widget::Row<'static, crate::app::Message> {
    let chars = line_display_chars(&line);
    let display_len = chars.len();
    let mut row_widget = row!().spacing(0).height(Length::Shrink);

    for col in 0..=display_len {
        if cursor_cols.contains(&col) {
            row_widget = row_widget.push(
                container(text(" "))
                    .width(Length::Fixed(2.0))
                    .height(Length::Fixed(16.0))
                    .style(move |_| iced::widget::container::Style {
                        background: Some(Background::Color(cursor_color)),
                        ..Default::default()
                    }),
            );
        }

        if col < display_len {
            let ch = chars[col];
            let selected = selections
                .iter()
                .any(|range| col >= range.start && col < range.end);
            row_widget = row_widget.push(
                container(text(ch.to_string()).size(14))
                    .padding(Padding::from([0.0, 0.0]))
                    .style(move |_| iced::widget::container::Style {
                        background: selected.then_some(Background::Color(selection_bg)),
                        text_color: selected.then_some(selection_fg),
                        ..Default::default()
                    }),
            );
        }
    }

    row_widget
}
