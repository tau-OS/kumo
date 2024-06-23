//! Logger backend module for building tracing_subscriber

const LOG_ENV_VAR: &str = "MOYA_LOG";

/// Helper function to build logger with our filters
pub fn build_logger() -> impl tracing::Subscriber + Send + Sync + 'static {
    let filter = std::env::var(LOG_ENV_VAR).unwrap_or_else(|_| "info".to_string());
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap()
        .add_directive(filter.parse().unwrap());
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .pretty()
        .finish()
}

/// Initialize logger
pub fn init() {
    tracing::subscriber::set_global_default(build_logger()).unwrap();
}