use clap::{ArgAction::Append, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add records of meals eaten
    Add(AddArgs),
    /// What you haven't eaten in the longest time
    What(WhatArgs),
    /// Plan meals to cook
    #[command(subcommand)]
    Plan(PlanCommand),
    /// Suggest a random meal
    Random(RandomArgs),
    #[command(subcommand)]
    /// Manage mrot configuration
    Config(ConfigCommand),
    /// Generate command completions
    #[command(subcommand)]
    Generate(GenerateCommand),
}

#[derive(Args)]
pub struct AddArgs {
    /// Meal to add (e.g. "rib eye steak")
    pub meal: String,
    /// Day to add this meal on
    #[arg(short, long, default_value = "today")]
    pub date: Option<String>,
}

#[derive(Args)]
pub struct WhatArgs {
    /// Limit to a number of suggestions (overrides config)
    #[arg(short, long)]
    pub number: Option<usize>,
    /// Ignore a certain meal (can use multiple times, overrides config)
    #[arg(short, long, action = Append)]
    pub ignore: Option<Vec<String>>,
}

#[derive(Subcommand)]
pub enum PlanCommand {
    /// Plan a meal for the future
    Add(PlanAddArgs),
    /// Show future meal plans
    Show(PlanShowArgs),
    /// Remove meal plans
    #[command(subcommand)]
    Remove(PlanRemoveCommand),
}

#[derive(Args)]
pub struct PlanAddArgs {
    /// Meal to plan (e.g. "rib eye steak")
    pub meal: String,
    /// Date to plan it on (e.g. "next Sunday")
    pub date: String,
}

#[derive(Args)]
pub struct PlanShowArgs {
    /// Limit the number of planned meals to show (overrides config)
    #[arg(short, long)]
    pub number: Option<usize>,
    /// Show planned meals up to this many days in the future (overrides config)
    #[arg(short, long)]
    pub days: Option<usize>,
}

#[derive(Subcommand)]
pub enum PlanRemoveCommand {
    /// Remove a given meal from planned meals
    Meal(PlanRemoveMealArgs),
    /// Remove the planned meals for a given date
    Date(PlanRemoveDateArgs),
    /// Remove the planned meals for a given time span
    Span(PlanRemoveSpanArgs),
}

#[derive(Args)]
pub struct PlanRemoveMealArgs {
    /// Meal to remove from the planned meals
    pub meal: String,
}

#[derive(Args)]
pub struct PlanRemoveDateArgs {
    /// Date for which the planned meals should be removed
    pub date: String,
}

#[derive(Args)]
pub struct PlanRemoveSpanArgs {
    /// Time span for which the planned meals should be removed
    pub span: String,
}

#[derive(Args)]
pub struct RandomArgs;

#[derive(Subcommand)]
pub enum ConfigCommand {
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
pub enum ConfigSetCommand {
    /// Set a limit when suggesting meals
    #[command(subcommand)]
    What(ConfigSetWhatCommand),
    /// Set the limits when showing planned meals
    #[command(subcommand)]
    Plan(ConfigSetPlanCommand),
}

#[derive(Subcommand)]
pub enum ConfigSetWhatCommand {
    /// Set the max number of meals suggested
    Number(ConfigSetWhatNumberArgs),
}

#[derive(Args)]
pub struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    pub number: usize,
}

#[derive(Subcommand)]
pub enum ConfigSetPlanCommand {
    /// Max number of planned meals to show
    Number(ConfigSetPlanNumberArgs),
    /// Limit of days in future for which to show planned meals
    Days(ConfigSetPlanDaysArgs),
}

#[derive(Args)]
pub struct ConfigSetPlanNumberArgs {
    /// Max number of planned meals to show
    pub number: usize,
}

#[derive(Args)]
pub struct ConfigSetPlanDaysArgs {
    /// Planned meals that lie more than this many days in the future won't be shown
    pub days: usize,
}

#[derive(Subcommand)]
pub enum ConfigGetCommand {
    /// See the configuration for meal suggestions
    #[command(subcommand)]
    What(ConfigGetWhatCommand),
    /// See the configuration for meal plans
    #[command(subcommand)]
    Plan(ConfigGetPlanCommand),
}

#[derive(Subcommand)]
pub enum ConfigGetWhatCommand {
    /// Max number of meals to suggest
    Number(ConfigGetWhatNumberArgs),
}

#[derive(Args)]
pub struct ConfigGetWhatNumberArgs;

#[derive(Subcommand)]
pub enum ConfigGetPlanCommand {
    /// Max number of planned meals to show
    Number(ConfigGetPlanNumberArgs),
    /// Planned meals that lie more than this many days in the future won't be shown
    Days(ConfigGetPlanDaysArgs),
}

#[derive(Args)]
pub struct ConfigGetPlanNumberArgs;

#[derive(Args)]
pub struct ConfigGetPlanDaysArgs;

#[derive(Subcommand)]
pub enum ConfigIgnoreCommand {
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
pub struct ConfigIgnoreAddArgs {
    /// Meal to add to the ignore list
    pub meal: String,
}

#[derive(Args)]
pub struct ConfigIgnoreRemoveArgs {
    /// Meal to remove from the ignore list
    pub meal: String,
}

#[derive(Args)]
pub struct ConfigIgnoreShowArgs;

#[derive(Args)]
pub struct ConfigIgnoreClearArgs;

#[derive(Args)]
pub struct ConfigPathArgs;

#[derive(Subcommand)]
pub enum GenerateCommand {
    /// generate completion file for Bash
    Bash(GenerateBashArgs),
    /// generate completion file for Elvish
    Elvish(GenerateElvishArgs),
    /// generate completion file for Fish
    Fish(GenerateFishArgs),
    /// generate completion file for PowerShell
    PowerShell(GeneratePowerShellArgs),
    /// generate completion file for Zsh
    Zsh(GenerateZshArgs),
}

#[derive(Args)]
pub struct GenerateBashArgs;

#[derive(Args)]
pub struct GenerateElvishArgs;

#[derive(Args)]
pub struct GenerateFishArgs;

#[derive(Args)]
pub struct GeneratePowerShellArgs;

#[derive(Args)]
pub struct GenerateZshArgs;
