//! CLI for mrot

#[cfg(test)]
use clap::CommandFactory;
use clap::{
    ArgAction::{Append, SetTrue},
    Args, Parser, Subcommand,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Add records of meals eaten
    Add(AddArgs),
    /// Parse date
    ParseDate(ParseDateArgs),
    /// What you haven't eaten in the longest time
    What(WhatArgs),
    /// Suggest a random meal
    Random(RandomArgs),
    /// Show recorded meals
    Show(ShowArgs),
    /// Searches records of a given meal
    When(WhenArgs),
    /// Show unique recorded meals
    Unique(UniqueArgs),
    /// Remove records of meals
    Remove(RemoveArgs),
    /// Rename meals
    Rename(RenameArgs),
    /// Manage mrot configuration
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Generate command completions
    #[command(subcommand)]
    Generate(GenerateCommand),
    /// Show paths to data files
    #[command(subcommand)]
    Path(PathCommand),
}

#[derive(Args)]
pub(crate) struct AddArgs {
    /// Meal to add (e.g. "rib eye steak")
    pub(crate) meal: String,
    /// Day to add this meal on
    #[arg(short, long, action = Append)]
    pub(crate) date: Option<Vec<String>>,
}

#[derive(Args)]
pub(crate) struct ParseDateArgs {
    /// Date string to parse
    pub(crate) date: String,
    /// Output dates as Unix timestamps
    #[arg(short = 't', long, action = SetTrue)]
    pub(crate) output_timestamp: bool,
}

#[derive(Args)]
pub(crate) struct WhatArgs {
    /// Limit to a number of suggestions (overrides config)
    #[arg(short, long)]
    pub(crate) number: Option<u64>,
    /// Ignore a certain meal (can use multiple times, overrides config)
    #[arg(short, long, action = Append)]
    pub(crate) ignore: Option<Vec<String>>,
    /// Ignore meals planned in this time span
    #[arg(short = 'p', long)]
    pub(crate) ignore_period: Option<String>,
    /// Consider also ignored meals
    #[arg(short = 'I', long, action = SetTrue, conflicts_with = "ignore")]
    pub(crate) no_ignore: bool,
    /// Disregard planned meals
    #[arg(short = 'P', long, action = SetTrue, conflicts_with = "ignore_period")]
    pub(crate) no_ignore_period: bool,
}

#[derive(Args)]
pub(crate) struct RandomArgs;

#[derive(Args)]
pub(crate) struct ShowArgs {
    /// Date or date range to show meals from (overrides config)
    pub(crate) range: Option<String>,
}

#[derive(Args)]
pub(crate) struct WhenArgs {
    /// Meal to search for
    pub(crate) meal: String,
}

#[derive(Args)]
pub(crate) struct UniqueArgs;

#[derive(Args)]
pub(crate) struct RemoveArgs {
    /// Time range to show meals from
    pub(crate) range: String,
    /// meal to remove
    #[arg(short, long)]
    pub(crate) meal: Option<String>,
}

#[derive(Args)]
pub(crate) struct RenameArgs {
    /// Old name of a meal
    pub(crate) old_name: String,
    /// New name of a meal
    pub(crate) new_name: String,
    /// New name of a meal
    #[arg(short, long)]
    pub(crate) period: Option<String>,
}

#[derive(Subcommand)]
pub(crate) enum ConfigCommand {
    /// Set configuration values
    #[command(subcommand)]
    Set(ConfigSetCommand),
    /// Get configuration values
    #[command(subcommand)]
    Get(ConfigGetCommand),
    /// Manage the ignored meals list
    #[command(subcommand)]
    Ignore(ConfigIgnoreCommand),
}

#[derive(Subcommand)]
pub(crate) enum ConfigSetCommand {
    /// Set a limit when suggesting meals
    #[command(subcommand)]
    What(ConfigSetWhatCommand),
    /// Set the limits when showing planned meals
    Show(ConfigSetShowArgs),
}

#[derive(Subcommand)]
pub(crate) enum ConfigSetWhatCommand {
    /// Set the max number of meals suggested
    Number(ConfigSetWhatNumberArgs),
    /// Set the number of days to look ahead for planned meals
    IgnorePeriod(ConfigSetWhatIgnorePeriodArgs),
}

#[derive(Args)]
pub(crate) struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    pub(crate) number: u64,
}

#[derive(Args)]
pub(crate) struct ConfigSetWhatIgnorePeriodArgs {
    /// Optional string with a date expression describing the date or date range for look-ahead.
    /// Enter no string at all (not even an empty string) to configure no look-ahead.
    pub(crate) ignore_period: Option<String>,
}

#[derive(Args)]
pub(crate) struct ConfigSetShowArgs {
    /// Time range in which to show meals
    pub(crate) range: String,
}

#[derive(Subcommand)]
pub(crate) enum ConfigGetCommand {
    /// See the configuration for meal suggestions
    #[command(subcommand)]
    What(ConfigGetWhatCommand),
    /// See the configuration for showing meals
    Show(ConfigGetShowArgs),
}

#[derive(Subcommand)]
pub(crate) enum ConfigGetWhatCommand {
    /// Max number of meals to suggest
    Number(ConfigGetWhatNumberArgs),
    /// Days to look ahead for planned meals
    IgnorePeriod(ConfigGetWhatIgnorePeriodArgs),
}

#[derive(Args)]
pub(crate) struct ConfigGetWhatNumberArgs;

#[derive(Args)]
pub(crate) struct ConfigGetWhatIgnorePeriodArgs;

#[derive(Args)]
pub(crate) struct ConfigGetShowArgs;

#[derive(Subcommand)]
pub(crate) enum ConfigIgnoreCommand {
    /// Add a meal to the ignore list
    Add(ConfigIgnoreAddArgs),
    /// Remove a meal from the ignore list
    Remove(ConfigIgnoreRemoveArgs),
    /// Show the meals on the ignore list
    Show(ConfigIgnoreShowArgs),
    /// Clear the meal ignore list
    Clear(ConfigIgnoreClearArgs),
}

#[derive(Args)]
pub(crate) struct ConfigIgnoreAddArgs {
    /// Meal to add to the ignore list
    pub(crate) meal: String,
}

#[derive(Args)]
pub(crate) struct ConfigIgnoreRemoveArgs {
    /// Meal to remove from the ignore list
    pub(crate) meal: String,
}

#[derive(Args)]
pub(crate) struct ConfigIgnoreShowArgs;

#[derive(Args)]
pub(crate) struct ConfigIgnoreClearArgs;

#[derive(Subcommand)]
pub(crate) enum GenerateCommand {
    /// generate completion file for Bash
    Bash(GenerateBashArgs),
    /// generate completion file for Elvish
    Elvish(GenerateElvishArgs),
    /// generate completion file for Fish
    Fish(GenerateFishArgs),
    /// generate completion file for Nushell
    Nushell(GenerateNushellArgs),
    /// generate completion file for PowerShell
    PowerShell(GeneratePowerShellArgs),
    /// generate completion file for Zsh
    Zsh(GenerateZshArgs),
}

#[derive(Args)]
pub(crate) struct GenerateBashArgs;

#[derive(Args)]
pub(crate) struct GenerateElvishArgs;

#[derive(Args)]
pub(crate) struct GenerateFishArgs;

#[derive(Args)]
pub(crate) struct GenerateNushellArgs;

#[derive(Args)]
pub(crate) struct GeneratePowerShellArgs;

#[derive(Args)]
pub(crate) struct GenerateZshArgs;

#[derive(Subcommand)]
pub(crate) enum PathCommand {
    /// Show path to the configuration file
    Config(PathConfigArgs),
    /// Show path to the records file
    Records(PathRecordsArgs),
    /// Show path to the log file
    Log(PathLogArgs),
}

#[derive(Args)]
pub(crate) struct PathConfigArgs;

#[derive(Args)]
pub(crate) struct PathRecordsArgs;

#[derive(Args)]
pub(crate) struct PathLogArgs;

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
