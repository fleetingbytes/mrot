//! CLI for mrot

use clap::{
    ArgAction::{Append, SetTrue},
    Args, Parser, Subcommand,
};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
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
struct AddArgs {
    /// Meal to add (e.g. "rib eye steak")
    meal: String,
    /// Day to add this meal on
    #[arg(short, long, action = Append)]
    date: Option<Vec<String>>,
}

#[derive(Args)]
struct WhatArgs {
    /// Limit to a number of suggestions (overrides config)
    #[arg(short, long)]
    number: Option<usize>,
    /// Ignore a certain meal (can use multiple times, overrides config)
    #[arg(short, long, action = Append)]
    ignore: Option<Vec<String>>,
    /// Ignore meals planned in this time span
    #[arg(short, long)]
    look_ahead: Option<usize>,
    /// Include ignored meals
    #[arg(short = 'I', long, action = SetTrue, conflicts_with = "ignore")]
    no_ignore: bool,
    /// Disregard planned meals
    #[arg(short = 'L', long, action = SetTrue, conflicts_with = "look_ahead")]
    no_look_ahead: bool,
}

#[derive(Args)]
struct ShowArgs {
    /// Date or date range to show meals from (overrides config)
    range: Option<String>,
}

#[derive(Args)]
struct WhenArgs {
    /// Meal to search for
    meal: String,
}

#[derive(Args)]
struct RemoveArgs {
    /// Time range to show meals from
    range: String,
    /// meal to remove
    #[arg(short, long)]
    meal: Option<String>,
}

#[derive(Args)]
struct RandomArgs;

#[derive(Args)]
struct ParseDateArgs {
    /// Date string to parse
    date: String,
}

#[derive(Subcommand)]
enum ConfigCommand {
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
enum ConfigSetCommand {
    /// Set a limit when suggesting meals
    #[command(subcommand)]
    What(ConfigSetWhatCommand),
    /// Set the limits when showing planned meals
    Show(ConfigSetShowArgs),
}

#[derive(Subcommand)]
enum ConfigSetWhatCommand {
    /// Set the max number of meals suggested
    Number(ConfigSetWhatNumberArgs),
    /// Set the number of days to look ahead for planned meals
    LookAhead(ConfigSetWhatLookAheadArgs),
}

#[derive(Args)]
struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    number: usize,
}

#[derive(Args)]
struct ConfigSetWhatLookAheadArgs {
    /// Number of days after tomorrow to look-ahead for planned meals
    look_ahead: usize,
}

#[derive(Args)]
struct ConfigSetShowArgs {
    /// Time range in which to show meals
    range: String,
}

#[derive(Subcommand)]
enum ConfigGetCommand {
    /// See the configuration for meal suggestions
    #[command(subcommand)]
    What(ConfigGetWhatCommand),
    /// See the configuration for showing meals
    Show(ConfigGetShowArgs),
}

#[derive(Subcommand)]
enum ConfigGetWhatCommand {
    /// Max number of meals to suggest
    Number(ConfigGetWhatNumberArgs),
    /// Days to look-ahead for planned meals
    LookAhead(ConfigGetWhatLookAheadArgs),
}

#[derive(Args)]
struct ConfigGetWhatNumberArgs;

#[derive(Args)]
struct ConfigGetWhatLookAheadArgs;

#[derive(Args)]
struct ConfigGetShowArgs;

#[derive(Subcommand)]
enum ConfigIgnoreCommand {
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
struct ConfigIgnoreAddArgs {
    /// Meal to add to the ignore list
    meal: String,
}

#[derive(Args)]
struct ConfigIgnoreRemoveArgs {
    /// Meal to remove from the ignore list
    meal: String,
}

#[derive(Args)]
struct ConfigIgnoreShowArgs;

#[derive(Args)]
struct ConfigIgnoreClearArgs;

#[derive(Args)]
struct ConfigPathArgs;

#[derive(Subcommand)]
enum GenerateCommand {
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
struct GenerateBashArgs;

#[derive(Args)]
struct GenerateElvishArgs;

#[derive(Args)]
struct GenerateFishArgs;

#[derive(Args)]
struct GenerateNushellArgs;

#[derive(Args)]
struct GeneratePowerShellArgs;

#[derive(Args)]
struct GenerateZshArgs;

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
