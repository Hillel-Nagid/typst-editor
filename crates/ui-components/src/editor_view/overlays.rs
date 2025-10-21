//! Overlay components (autocomplete, hover info, parameter hints, etc.)
//!
//! Phase 3.1: Editor View Component Hierarchy

use gpui::*;
use editor_core::Position;

/// Overlay manager for popups and tooltips
pub struct Overlays {
    /// Active autocomplete popup
    pub autocomplete: Option<AutocompletePopup>,
    /// Active hover info
    pub hover: Option<HoverInfo>,
    /// Active parameter hints
    pub parameter_hints: Option<ParameterHints>,
    /// Active quick fixes menu
    pub quick_fixes: Option<QuickFixesMenu>,
}

impl Overlays {
    pub fn new() -> Self {
        Self {
            autocomplete: None,
            hover: None,
            parameter_hints: None,
            quick_fixes: None,
        }
    }

    /// Show autocomplete popup
    pub fn show_autocomplete(&mut self, popup: AutocompletePopup) {
        self.autocomplete = Some(popup);
    }

    /// Hide autocomplete popup
    pub fn hide_autocomplete(&mut self) {
        self.autocomplete = None;
    }

    /// Show hover info
    pub fn show_hover(&mut self, hover: HoverInfo) {
        self.hover = Some(hover);
    }

    /// Hide hover info
    pub fn hide_hover(&mut self) {
        self.hover = None;
    }

    /// Show parameter hints
    pub fn show_parameter_hints(&mut self, hints: ParameterHints) {
        self.parameter_hints = Some(hints);
    }

    /// Hide parameter hints
    pub fn hide_parameter_hints(&mut self) {
        self.parameter_hints = None;
    }

    /// Show quick fixes menu
    pub fn show_quick_fixes(&mut self, menu: QuickFixesMenu) {
        self.quick_fixes = Some(menu);
    }

    /// Hide quick fixes menu
    pub fn hide_quick_fixes(&mut self) {
        self.quick_fixes = None;
    }

    /// Hide all overlays
    pub fn hide_all(&mut self) {
        self.autocomplete = None;
        self.hover = None;
        self.parameter_hints = None;
        self.quick_fixes = None;
    }

    /// Check if any overlay is visible
    pub fn has_visible_overlay(&self) -> bool {
        self.autocomplete.is_some() ||
            self.hover.is_some() ||
            self.parameter_hints.is_some() ||
            self.quick_fixes.is_some()
    }
}

impl Default for Overlays {
    fn default() -> Self {
        Self::new()
    }
}

/// Autocomplete popup
#[derive(Debug, Clone)]
pub struct AutocompletePopup {
    /// Popup position
    pub position: Position,
    /// Completion items
    pub items: Vec<CompletionItem>,
    /// Selected item index
    pub selected: usize,
}

impl AutocompletePopup {
    pub fn new(position: Position, items: Vec<CompletionItem>) -> Self {
        Self {
            position,
            items,
            selected: 0,
        }
    }

    /// Select next item
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    /// Select previous item
    pub fn select_previous(&mut self) {
        if !self.items.is_empty() {
            self.selected = if self.selected == 0 {
                self.items.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Get selected item
    pub fn get_selected(&self) -> Option<&CompletionItem> {
        self.items.get(self.selected)
    }
}

/// Completion item
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

/// Completion kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Function,
    Variable,
    Keyword,
    Constant,
    Type,
    Module,
}

/// Hover information tooltip
#[derive(Debug, Clone)]
pub struct HoverInfo {
    /// Hover position
    pub position: Position,
    /// Markdown content
    pub content: String,
}

impl HoverInfo {
    pub fn new(position: Position, content: String) -> Self {
        Self { position, content }
    }
}

/// Parameter hints popup
#[derive(Debug, Clone)]
pub struct ParameterHints {
    /// Hints position
    pub position: Position,
    /// Signatures
    pub signatures: Vec<SignatureInfo>,
    /// Active signature index
    pub active_signature: usize,
    /// Active parameter index
    pub active_parameter: usize,
}

impl ParameterHints {
    pub fn new(position: Position, signatures: Vec<SignatureInfo>) -> Self {
        Self {
            position,
            signatures,
            active_signature: 0,
            active_parameter: 0,
        }
    }

    /// Get active signature
    pub fn get_active_signature(&self) -> Option<&SignatureInfo> {
        self.signatures.get(self.active_signature)
    }
}

/// Signature information
#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub label: String,
    pub parameters: Vec<ParameterInfo>,
    pub documentation: Option<String>,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub label: String,
    pub documentation: Option<String>,
}

/// Quick fixes menu
#[derive(Debug, Clone)]
pub struct QuickFixesMenu {
    /// Menu position
    pub position: Position,
    /// Available actions
    pub actions: Vec<CodeAction>,
    /// Selected action index
    pub selected: usize,
}

impl QuickFixesMenu {
    pub fn new(position: Position, actions: Vec<CodeAction>) -> Self {
        Self {
            position,
            actions,
            selected: 0,
        }
    }

    /// Select next action
    pub fn select_next(&mut self) {
        if !self.actions.is_empty() {
            self.selected = (self.selected + 1) % self.actions.len();
        }
    }

    /// Select previous action
    pub fn select_previous(&mut self) {
        if !self.actions.is_empty() {
            self.selected = if self.selected == 0 {
                self.actions.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Get selected action
    pub fn get_selected(&self) -> Option<&CodeAction> {
        self.actions.get(self.selected)
    }
}

/// Code action
#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
}

/// Code action kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    SourceAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlays_creation() {
        let overlays = Overlays::new();
        assert!(overlays.autocomplete.is_none());
        assert!(overlays.hover.is_none());
        assert!(overlays.parameter_hints.is_none());
        assert!(overlays.quick_fixes.is_none());
        assert!(!overlays.has_visible_overlay());
    }

    #[test]
    fn test_autocomplete_show_hide() {
        let mut overlays = Overlays::new();
        let popup = AutocompletePopup::new(Position::new(0, 0), vec![]);
        overlays.show_autocomplete(popup);
        assert!(overlays.autocomplete.is_some());
        assert!(overlays.has_visible_overlay());
        overlays.hide_autocomplete();
        assert!(overlays.autocomplete.is_none());
    }

    #[test]
    fn test_autocomplete_navigation() {
        let items = vec![
            CompletionItem {
                label: "item1".to_string(),
                kind: CompletionKind::Function,
                detail: None,
                documentation: None,
            },
            CompletionItem {
                label: "item2".to_string(),
                kind: CompletionKind::Variable,
                detail: None,
                documentation: None,
            }
        ];
        let mut popup = AutocompletePopup::new(Position::new(0, 0), items);

        assert_eq!(popup.selected, 0);
        popup.select_next();
        assert_eq!(popup.selected, 1);
        popup.select_next();
        assert_eq!(popup.selected, 0); // Wrap around

        popup.select_previous();
        assert_eq!(popup.selected, 1); // Wrap around backwards
    }

    #[test]
    fn test_hover_info() {
        let hover = HoverInfo::new(Position::new(1, 5), "Test documentation".to_string());
        assert_eq!(hover.position, Position::new(1, 5));
        assert_eq!(hover.content, "Test documentation");
    }

    #[test]
    fn test_parameter_hints() {
        let sigs = vec![SignatureInfo {
            label: "func(a: int, b: int)".to_string(),
            parameters: vec![],
            documentation: None,
        }];
        let hints = ParameterHints::new(Position::new(0, 0), sigs);
        assert_eq!(hints.active_signature, 0);
        assert_eq!(hints.active_parameter, 0);
        assert!(hints.get_active_signature().is_some());
    }

    #[test]
    fn test_quick_fixes_menu() {
        let actions = vec![
            CodeAction {
                title: "Fix import".to_string(),
                kind: CodeActionKind::QuickFix,
            },
            CodeAction {
                title: "Extract variable".to_string(),
                kind: CodeActionKind::Refactor,
            }
        ];
        let mut menu = QuickFixesMenu::new(Position::new(0, 0), actions);

        assert_eq!(menu.selected, 0);
        menu.select_next();
        assert_eq!(menu.selected, 1);
        assert_eq!(menu.get_selected().unwrap().title, "Extract variable");
    }

    #[test]
    fn test_hide_all() {
        let mut overlays = Overlays::new();
        overlays.show_autocomplete(AutocompletePopup::new(Position::new(0, 0), vec![]));
        overlays.show_hover(HoverInfo::new(Position::new(0, 0), "test".to_string()));
        assert!(overlays.has_visible_overlay());

        overlays.hide_all();
        assert!(!overlays.has_visible_overlay());
    }
}
