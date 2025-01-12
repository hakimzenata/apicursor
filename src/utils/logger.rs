use serde::Serialize;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;
#[derive(Debug, Serialize)]
struct AppLogFormat {
    timestamp: String,
    message: String,
}

impl AppLogFormat {
    pub fn new(timestamp: String, message: String) -> Self {
        Self { timestamp, message }
    }
}

pub fn init_logger() {
    // Initialize with JSON format
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE)
        .with_target(false)
        .without_time()
        .with_level(false)
        .init();

    let log_format = AppLogFormat::new(
        chrono::Utc::now().to_rfc3339(),
        "Logger initialized! 🚀".to_string(),
    );
    info!(message = serde_json::to_string(&log_format).unwrap());
}

pub fn log_info_app(message: String) {
    let log_format = AppLogFormat::new(chrono::Utc::now().to_rfc3339(), message);
    info!(message = serde_json::to_string(&log_format).unwrap());
}

// pub fn log_error_app(message: String) {
//     let log_format = AppLogFormat::new(chrono::Utc::now().to_rfc3339(), message);
//     error!(message = serde_json::to_string(&log_format).unwrap());
// }

// pub fn log_warn_app(message: String) {
//     let log_format = AppLogFormat::new(chrono::Utc::now().to_rfc3339(), message);
//     warn!(message = serde_json::to_string(&log_format).unwrap());
// }

// pub fn log_debug_app(message: String) {
//     let log_format = AppLogFormat::new(chrono::Utc::now().to_rfc3339(), message);
//     debug!(message = serde_json::to_string(&log_format).unwrap());
// }

// pub fn log_trace_app(message: String) {
//     let log_format = AppLogFormat::new(chrono::Utc::now().to_rfc3339(), message);
//     trace!(message = serde_json::to_string(&log_format).unwrap());
// }
