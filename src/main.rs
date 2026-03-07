use anyhow::Result;
use tracing_subscriber;
use ui::run;
fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber
        ::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter
                ::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    tracing::info!("Starting Typst Studio");

    // Initialize Iced application
    run()?;

    Ok(())
}
