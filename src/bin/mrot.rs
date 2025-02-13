//!~Meal Rotator
//!
//!
#![deny(missing_docs)]

use color_eyre::eyre::Report;
use mrot::error::Error;
use tracing::{error, instrument, trace};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn init_tracing() -> () {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MROT_LOG_LEVEL"))
        .init();
}

#[instrument]
fn main() -> Result<(), Report> {
    init_tracing();
    trace!("Tracing initialized");
    match mrot::cli::run() {
        Ok(_) => {
            trace!("cli::run ran without errors");
            Ok(())
        }
        Err(e) if matches!(e, Error::TimeSpanNotSupported) => {
            trace!("cli::run ran with Error::TimeSpanNotSupported");
            error!("{}", e);
            Ok(())
        }
        Err(e) => {
            trace!("cli::run ran with an error");
            Err(Report::new(e))
        }
    }
}
