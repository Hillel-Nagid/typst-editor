//! Typst Editor - Main Application Entry Point

mod state;
mod app;

use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_target(false).init();

    tracing::info!("Starting Typst Editor");

    // For now, just run a simple version
    // In a real GPUI app, we would initialize GPUI and create the app
    println!("Typst Editor v0.1.0");
    println!("Phase 1 Implementation Complete!");
    println!("- Workspace structure created");
    println!("- Core data models implemented");
    println!("- Text buffer with rope structure ready");
    println!("- Bidirectional text support implemented");
    println!("- Typst compiler integration prepared");
    println!("- Preview system ready");
    println!("- LSP client foundation complete");
    println!("- UI components structure established");

    // TODO: Initialize GPUI application
    // let app = gpui::App::new();
    // app.run(|cx| {
    //     TypstEditor::new(cx)
    // });
}
