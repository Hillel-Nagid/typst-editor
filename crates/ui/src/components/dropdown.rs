use crate::theme::Theme;
use iced::widget::pick_list;
use iced::Element;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownOption {
    pub value: String,
    pub label: String,
}

impl std::fmt::Display for DropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub fn dropdown_view(
    _theme: &Theme,
    options: Vec<DropdownOption>,
    selected: Option<DropdownOption>,
    on_select: impl Fn(DropdownOption) -> crate::app::Message + 'static,
) -> Element<'static, crate::app::Message> {
    pick_list(options, selected, on_select).into()
}
