use crate::theme::Theme;
use iced::widget::text;
use iced::Element;

#[derive(Clone, Copy)]
pub enum IconSize {
    Small, // 12px
    Medium, // 16px
    Large, // 24px
}

impl IconSize {
    fn to_px(&self) -> u16 {
        match self {
            IconSize::Small => 12,
            IconSize::Medium => 16,
            IconSize::Large => 24,
        }
    }
}

pub enum IconType {
    File,
    Folder,
    FolderOpen,
    Save,
    Open,
    Close,
    Settings,
    Search,
    Error,
    Warning,
    Info,
    Success,
    ChevronRight,
    ChevronDown,
}

impl IconType {
    fn to_emoji(&self) -> &'static str {
        match self {
            IconType::File => "📄",
            IconType::Folder => "📁",
            IconType::FolderOpen => "📂",
            IconType::Save => "💾",
            IconType::Open => "📂",
            IconType::Close => "✕",
            IconType::Settings => "⚙️",
            IconType::Search => "🔍",
            IconType::Error => "❌",
            IconType::Warning => "⚠️",
            IconType::Info => "ℹ️",
            IconType::Success => "✓",
            IconType::ChevronRight => "›",
            IconType::ChevronDown => "⌄",
        }
    }
}

pub struct Icon {
    icon_type: IconType,
    size: IconSize,
}

impl Icon {
    pub fn new(icon_type: IconType, size: IconSize) -> Self {
        Self { icon_type, size }
    }

    pub fn view(self, theme: &Theme) -> Element<'static, crate::app::Message> {
        let color = theme.parse_color(&theme.foreground.editor);
        text(self.icon_type.to_emoji())
            .size(self.size.to_px())
            .color(color)
            .into()
    }
}
