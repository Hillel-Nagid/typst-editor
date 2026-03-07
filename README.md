# Typst Studio

A high-performance native cross-platform text editor for Typst documents with bidirectional text support, live preview, and full LSP integration, built with Iced.

## Features (Phase 1 & 2 Implementation)

### ✅ Completed

- **Core Architecture**

  - Cargo workspace structure with modular crates
  - Document and project model implementations
  - Configuration system with hierarchical settings
  - Application state management with thread-safe access

- **UI Foundation**

  - Iced-based application with native Rust UI rendering
  - Window management system
  - Dark and light theme support with customizable colors
  - Responsive layout with panel management

- **UI Components**
  - Main editor panel with line numbers and gutter
  - PDF preview pane (stub for Phase 7)
  - File explorer sidebar
  - Navigation bar with menu system
  - Console/diagnostics panel
  - Status bar with document information
  - Reusable components (buttons, splitters, scrollbars)

### 🚧 Upcoming (Later Phases)

- **Phase 3**: Text buffer with rope data structure, multi-cursor support, undo/redo
- **Phase 4**: Bidirectional text support (RTL/LTR)
- **Phase 5**: LSP integration with Typst language server
- **Phase 6**: Typst compiler integration
- **Phase 7**: PDF preview rendering
- **Phase 8**: Advanced features (search/replace, folding, snippets)
- **Phase 9**: Settings UI and configuration management
- **Phase 10**: Performance optimization and polish

## Project Structure

```
typst-studio/
├── crates/
│   ├── editor_core/       # Core data models and logic
│   │   ├── config.rs      # Configuration system
│   │   ├── document.rs    # Document model
│   │   ├── project.rs     # Project model
│   │   ├── buffer.rs      # Text buffer (rope-based)
│   │   ├── selection.rs   # Cursor and selection
│   │   └── state.rs       # Application state management
│   ├── typst_integration/ # Typst compiler and LSP (stubs)
│   ├── preview/           # PDF preview rendering (stubs)
│   └── ui/                # Iced-based UI components
│       ├── app.rs         # Application entry point
│       ├── theme.rs       # Theme system
│       ├── workspace.rs   # Main window layout
│       ├── components/    # Reusable UI components
│       ├── editor.rs      # Editor panel
│       ├── preview_pane.rs # Preview panel
│       ├── sidebar.rs     # File explorer
│       ├── navbar.rs      # Navigation bar
│       └── console.rs     # Console/diagnostics
└── src/
    └── main.rs            # Application entry point
```

## Building from Source

### Prerequisites

- Rust 1.75 or later
- Git

### Build Steps

```bash
# Clone the repository
git clone https://github.com/Hillel-Nagid/typst-studio.git
cd typst-studio

# Build the project
cargo build --release

# Run the editor
cargo run --release
```

### Development Build

```bash
# Build and run in debug mode
cargo run

# Enable logging
RUST_LOG=info cargo run
```

## Configuration

Configuration files are stored in platform-specific locations:

- **Windows**: `%APPDATA%\typst-studio\config.toml`
- **macOS**: `~/Library/Application Support/typst-studio/config.toml`
- **Linux**: `~/.config/typst-studio/config.toml`

Example configuration:

```toml
[editor]
font_family = "Fira Code"
font_size = 14
line_height = 1.5
tab_size = 4
insert_spaces = true
line_numbers = true
minimap = true

[appearance]
theme = "dark"  # or "light"
ui_scale = 1.0

[compiler]
auto_compile_on_save = true
auto_compile_on_change = true
compilation_delay = 500
```

## Architecture

### State Management

The application uses a hierarchical state management system:

- **ApplicationState**: Global application state, window management
- **WorkspaceState**: Per-workspace state, open documents
- **EditorState**: Per-document state, cursor positions, content

All state is wrapped in `Arc<RwLock<T>>` for thread-safe concurrent access.

### Threading Model

- **Main Thread**: UI rendering and event handling (Iced)
- **LSP Thread**: Language server communication (Phase 5)
- **Compiler Thread**: Document compilation (Phase 6)
- **File Watcher Thread**: Filesystem monitoring (Phase 6)

### Event Flow

```
User Input → Iced Messages → Action Handlers → State Updates → UI Re-render
```

## Development

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p editor_core
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting pull requests.

## Roadmap

See [plan.md](plan.md) for the complete implementation roadmap.

## Acknowledgments

- Built with [Iced](https://github.com/iced-rs/iced) framework
- [Typst](https://github.com/typst/typst) document processor
- Inspired by modern code editors (VS Code, Zed, etc.)
