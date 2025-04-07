//! This crate is a CLI application for the [libmrot] library.
//!
//! # Meal Rotator
//!
//! Helps you to rotate through the meals you cook by suggesting what to cook next.
//!
//! ## Why mrot exists
//!
//! The Meal Rotator, or *mrot* for short, is a tool to record the kinds of meals you and your family consume and the dates when you consume them. When you ask mrot to give you suggestions what to cook next, the meals which you haven't had in the longest time become the likely candidates but your food preferences and meal plans for the future are also considered. Also mrot limits the number suggested meals to as few or as many as you feel comfortable with.
//!
//! ## Quick Start
//!
//! By using the *add* subcommand you tell mrot what meal your family consumed on what date. The same subcommand enables you to plan some meals for the days to come, if you wish to do so.
//!
//! ### Examples:
//!
//! Record that you had spaghetti today. The cli option `--date` defaults to `"tomorrow"`, so there is no need to explicitly use it here:
//! ```sh
//! $ mrot add spaghetti
//! ```
//!
//! Record that you are going to have spaghetti tomorrow:
//! ```sh
//! $ mrot add spaghetti --date tomorrow
//! ```
//!
//! To remove meal records, use the *remove* subcommand:
//! ```sh
//! $ mrot remove "this week"
//! ```
//!
//! ### Getting Meal Suggestions
//!
//! To demonstrate how mrot suggests meals, we will need to have some data recorded to work with:
//! ```sh
//! $ mrot add spaghetti --date "from March 1 through March 2"
//! $ mrot add "meat balls" --date "from March 3 through March 4"
//! $ mrot add pizza --date "March 5"
//! $ mrot add steak --date "March 6"
//! $ mrot add "lentils and wieners" --date "from March 8 through March 9"
//!
//! $ # assuming today is March 9
//! $ # plan to have meat balls on March 11
//! $ mrot add "meat balls" --date "one day after tomorrow"
//!
//! $ # let's see what meal we could have next
//! $ mrot what
//! spaghetti
//! pizza
//! steak
//! ```
//!
//! Notice how *meat balls* were not suggested even though you haven't had them for a longer time than a pizza or a steak. That is because you already planned them in the near future.
//!
//!
//! When you run the `what` subcommand mrot tries to suggest you the meals which you have not consumed for the longest time. If a meal from long ago matches a meal planned in the near future (by default in the next twelve days starting tomorrow), it is not suggested in order to avoid having the same meal again too soon. This is called the *ignore period* option and you can configure it or disable it entirely. Independent of this you can pass the names of any meals which you do not want to be suggested, see the *ignore* option below.
//!
//! The procedure which mrot runs internally is something like this:
//!
//! * for each unique recorded meal, look up the date when it was last consumed
//! * filter out the meals which are on the ignore list
//! * filter out the meals which are planned and recorded in advance within the ignore period
//! * limit the number of suggestions according to your configuration or the CLI option
//!
//! ## Documentation
//!
//! See the full documentation in the [repository](https://github.com/fleetingbytes/mrot/)'s readme.

pub(crate) mod cli;
mod config;
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

/// Type alias for results with mrot's [Error].
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
