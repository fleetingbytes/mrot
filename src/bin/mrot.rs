//!~Meal Rotator
//!
//!
#![deny(missing_docs)]

use color_eyre::eyre::Report;
use mrot::cli::translate_cli_to_api;

fn main() -> Result<(), Report> {
    translate_cli_to_api()?;
    Ok(())
}
