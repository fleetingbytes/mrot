//!~Meal Rotator
//!
//! This is a CLI application. It uses [libmrot] and [mrot_config]. See the repository's readme.
pub(crate) mod cli;
mod error;
mod run;

pub(crate) use crate::error::Error;
use directories::ProjectDirs;
use tracing::error;
use tracing_appender::non_blocking;
use tracing_subscriber::{filter::EnvFilter, fmt, fmt::format::FmtSpan, prelude::*};

pub(crate) const LOG_FILE: &str = "trace.log";
pub(crate) const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const LOG_LEVEL_ENV_VAR: &str = concat!(env!("PKG_NAME_UPPERCASE"), "_LOG_LEVEL");

/// Type alias for results with libmrot's [Error].
pub(crate) type Result<T> = std::result::Result<T, Error>;

fn init_tracing() -> Result<Vec<impl Drop>> {
    let (non_blocking_stderr, stderr_guard) = non_blocking(std::io::stderr());
    let stderr_log_layer = fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking_stderr)
        .without_time()
        .pretty();

    let dirs =
        ProjectDirs::from("", "", PKG_NAME).ok_or(Error::NoDirectory("directories::ProjectDirs: no valid home directory path could be retrieved from the operating system".to_string()))?;
    let log_file = dirs.data_dir().join(LOG_FILE);
    let log_file_writer = std::fs::File::create(log_file)?;
    let (non_blocking_file, file_guard) = non_blocking(log_file_writer);
    let file_log_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(stderr_log_layer.with_filter(EnvFilter::from_env(LOG_LEVEL_ENV_VAR)))
        .with(file_log_layer)
        .init();

    Ok(vec![stderr_guard, file_guard])
}

fn main() -> Result<()> {
    let _guards = init_tracing()?;
    match run::run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    }
}
