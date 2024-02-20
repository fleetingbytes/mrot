//!~Meal Rotator
//!
//!
#![deny(missing_docs)]

use color_eyre::eyre::Report;
use mrot::error::Error;

fn main() -> Result<(), Report> {
    match mrot::cli::run() {
        Ok(_) => Ok(()),
        Err(e) if matches!(e, Error::TimeSpanNotSupported) => {
            eprintln!("{}", e);
            Ok(())
        }
        Err(e) => Err(Report::new(e)),
    }
}
