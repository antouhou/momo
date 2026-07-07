use tracing_subscriber::{EnvFilter, util::SubscriberInitExt};

#[cfg(target_os = "android")]
use tracing_subscriber::layer::SubscriberExt;

const DEFAULT_FILTER: &str = "info";

#[derive(Debug, thiserror::Error)]
pub enum TracingInitializationError {
    #[cfg(target_os = "android")]
    #[error("failed to initialize the Android tracing layer")]
    AndroidLayer(#[source] std::io::Error),
    #[error("failed to install the global tracing subscriber")]
    GlobalSubscriber(#[from] tracing_subscriber::util::TryInitError),
}

/// Installs the process-wide tracing subscriber.
///
/// `RUST_LOG` controls the filter and defaults to `info`. Android applications
/// write to logcat using `application_name` as their tag; other platforms write
/// to standard output.
pub fn initialize_tracing(application_name: &str) -> Result<(), TracingInitializationError> {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_FILTER.to_string());

    initialize_platform_tracing(application_name, EnvFilter::new(filter))
}

#[cfg(target_os = "android")]
fn initialize_platform_tracing(
    application_name: &str,
    filter: EnvFilter,
) -> Result<(), TracingInitializationError> {
    let android_layer = tracing_android::layer(application_name)
        .map_err(TracingInitializationError::AndroidLayer)?;

    tracing_subscriber::registry()
        .with(filter)
        .with(android_layer)
        .try_init()?;

    Ok(())
}

#[cfg(not(target_os = "android"))]
fn initialize_platform_tracing(
    _application_name: &str,
    filter: EnvFilter,
) -> Result<(), TracingInitializationError> {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .finish()
        .try_init()?;

    Ok(())
}
