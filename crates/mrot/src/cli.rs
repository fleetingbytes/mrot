//! CLI for mrot

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
    /// What you haven't eaten in the longest time
    What(WhatArgs),
    /// Show recorded meals
    Show(ShowArgs),
    /// Searches records of a given meal
    When(WhenArgs),
    /// Remove records of meals
    Remove(RemoveArgs),
    /// Suggest a random meal
    Random(RandomArgs),
    #[command(subcommand)]
    /// Manage mrot configuration
    Config(ConfigCommand),
    /// Generate command completions
    #[command(subcommand)]
    Generate(GenerateCommand),
    /// Parse date
    ParseDate(ParseDateArgs),
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
pub(crate) struct WhatArgs {
    /// Limit to a number of suggestions (overrides config)
    #[arg(short, long)]
    pub(crate) number: Option<usize>,
    /// Ignore a certain meal (can use multiple times, overrides config)
    #[arg(short, long, action = Append)]
    pub(crate) ignore: Option<Vec<String>>,
    /// Ignore meals planned in this time span
    #[arg(short, long)]
    pub(crate) look_ahead: Option<usize>,
    /// Include ignored meals
    #[arg(short = 'I', long, action = SetTrue, conflicts_with = "ignore")]
    pub(crate) no_ignore: bool,
    /// Disregard planned meals
    #[arg(short = 'L', long, action = SetTrue, conflicts_with = "look_ahead")]
    pub(crate) no_look_ahead: bool,
}

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
pub(crate) struct RemoveArgs {
    /// Time range to show meals from
    pub(crate) range: String,
    /// meal to remove
    #[arg(short, long)]
    pub(crate) meal: Option<String>,
}

#[derive(Args)]
pub(crate) struct RandomArgs;

#[derive(Args)]
pub(crate) struct ParseDateArgs {
    /// Date string to parse
    pub(crate) date: String,
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
    /// Show the path to the configuration file
    Path(ConfigPathArgs),
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
    LookAhead(ConfigSetWhatLookAheadArgs),
}

#[derive(Args)]
pub(crate) struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    pub(crate) number: usize,
}

#[derive(Args)]
pub(crate) struct ConfigSetWhatLookAheadArgs {
    /// Number of days after tomorrow to look-ahead for planned meals
    pub(crate) look_ahead: usize,
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
    /// Days to look-ahead for planned meals
    LookAhead(ConfigGetWhatLookAheadArgs),
}

#[derive(Args)]
pub(crate) struct ConfigGetWhatNumberArgs;

#[derive(Args)]
pub(crate) struct ConfigGetWhatLookAheadArgs;

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

#[derive(Args)]
pub(crate) struct ConfigPathArgs;

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

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
