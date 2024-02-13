//!~Meal Rotator
//!
//!
#![deny(missing_docs)]

use color_eyre::eyre::Report;

fn main() -> Result<(), Report> {
    mrot::cli::run()?;
    Ok(())
}
