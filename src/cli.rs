//! CLI for mrot

#![deny(missing_docs)]

use crate::config::MrotConfig;
use crate::error::Error;
use crate::{add_meal, add_plan, open_storage};
use clap::{ArgAction::Append, Args, Command as ClapCommand, CommandFactory, Parser, Subcommand};
use clap_complete::{generate as generate_completions, shells, Generator};
use clap_complete_nushell::Nushell;
use std::io;

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
struct AddArgs {
    /// Meal to add (e.g. "rib eye steak")
    meal: String,
    /// Day to add this meal on
    #[arg(short, long, default_value = "today")]
    date: Option<String>,
}

#[derive(Args)]
struct WhatArgs {
    /// Limit to a number of suggestions (overrides config)
    #[arg(short, long)]
    number: Option<usize>,
    /// Ignore a certain meal (can use multiple times, overrides config)
    #[arg(short, long, action = Append)]
    ignore: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum PlanCommand {
    /// Plan a meal for the future
    Add(PlanAddArgs),
    /// Show future meal plans
    Show(PlanShowArgs),
    /// Remove meal plans
    #[command(subcommand)]
    Remove(PlanRemoveCommand),
}

#[derive(Args)]
struct PlanAddArgs {
    /// Meal to plan (e.g. "rib eye steak")
    meal: String,
    /// Date to plan it on (e.g. "next Sunday")
    date: String,
}

#[derive(Args)]
struct PlanShowArgs {
    /// Limit the number of planned meals to show (overrides config)
    #[arg(short, long)]
    number: Option<usize>,
    /// Show planned meals up to this many days in the future (overrides config)
    #[arg(short, long)]
    days: Option<usize>,
}

#[derive(Subcommand)]
enum PlanRemoveCommand {
    /// Remove a given meal from planned meals
    Meal(PlanRemoveMealArgs),
    /// Remove the planned meals for a given date
    Date(PlanRemoveDateArgs),
    /// Remove the planned meals for a given time span
    Span(PlanRemoveSpanArgs),
}

#[derive(Args)]
struct PlanRemoveMealArgs {
    /// Meal to remove from the planned meals
    meal: String,
}

#[derive(Args)]
struct PlanRemoveDateArgs {
    /// Date for which the planned meals should be removed
    date: String,
}

#[derive(Args)]
struct PlanRemoveSpanArgs {
    /// Time span for which the planned meals should be removed
    span: String,
}

#[derive(Args)]
struct RandomArgs;

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
    #[command(subcommand)]
    Plan(ConfigSetPlanCommand),
}

#[derive(Subcommand)]
enum ConfigSetWhatCommand {
    /// Set the max number of meals suggested
    Number(ConfigSetWhatNumberArgs),
}

#[derive(Args)]
struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    number: usize,
}

#[derive(Subcommand)]
enum ConfigSetPlanCommand {
    /// Max number of planned meals to show
    Number(ConfigSetPlanNumberArgs),
    /// Limit of days in future for which to show planned meals
    Days(ConfigSetPlanDaysArgs),
}

#[derive(Args)]
struct ConfigSetPlanNumberArgs {
    /// Max number of planned meals to show
    number: usize,
}

#[derive(Args)]
struct ConfigSetPlanDaysArgs {
    /// Planned meals that lie more than this many days in the future won't be shown
    days: usize,
}

#[derive(Subcommand)]
enum ConfigGetCommand {
    /// See the configuration for meal suggestions
    #[command(subcommand)]
    What(ConfigGetWhatCommand),
    /// See the configuration for meal plans
    #[command(subcommand)]
    Plan(ConfigGetPlanCommand),
}

#[derive(Subcommand)]
enum ConfigGetWhatCommand {
    /// Max number of meals to suggest
    Number(ConfigGetWhatNumberArgs),
}

#[derive(Args)]
struct ConfigGetWhatNumberArgs;

#[derive(Subcommand)]
enum ConfigGetPlanCommand {
    /// Max number of planned meals to show
    Number(ConfigGetPlanNumberArgs),
    /// Planned meals that lie more than this many days in the future won't be shown
    Days(ConfigGetPlanDaysArgs),
}

#[derive(Args)]
struct ConfigGetPlanNumberArgs;

#[derive(Args)]
struct ConfigGetPlanDaysArgs;

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

const CONFIG_FILE_NAME: &str = "config";

/// Parses the CLI commands and makes the required API calls to execute them
pub fn run() -> Result<(), Error> {
    let app_name = Cli::command().get_name().to_string();
    let mut cfg: MrotConfig = confy::load(&app_name, CONFIG_FILE_NAME)?;
    let cli = Cli::parse();
    match &cli.command {
        Command::Add(add) => {
            let date = match &add.date {
                Some(d) => d,
                None => "today",
            };
            let storage = open_storage()?;
            add_meal(&add.meal, &date, &storage)?;
        }
        Command::What(what) => {
            if let Some(ref number) = what.number {
                println!("what number is {}", number);
            }
            if let Some(ref ignore) = what.ignore {
                println!("what ignore is {:?}", ignore);
            }
            println!("what is run");
        }
        Command::Plan(plan) => {
            let storage = open_storage()?;
            match plan {
                PlanCommand::Add(plan_add) => {
                    add_plan(&plan_add.meal, &plan_add.date, &storage)?;
                }
                PlanCommand::Show(plan_show) => {
                    if let Some(ref number) = plan_show.number {
                        println!("plan show number is {}", number);
                    }
                    if let Some(ref days) = plan_show.days {
                        println!("plan show days is {}", days);
                    }
                    println!("plan show is run");
                }
                PlanCommand::Remove(plan_remove) => match plan_remove {
                    PlanRemoveCommand::Meal(plan_remove_meal) => {
                        println!("plan remove meal is {}", plan_remove_meal.meal);
                    }
                    PlanRemoveCommand::Date(plan_remove_date) => {
                        println!("plan remove date is {}", plan_remove_date.date);
                    }
                    PlanRemoveCommand::Span(plan_remove_span) => {
                        println!("plan remove span is {}", plan_remove_span.span);
                    }
                },
            }
        }
        Command::Random(_) => {
            println!("random is run");
        }
        Command::Config(config) => {
            let config_path = confy::get_configuration_file_path(&app_name, CONFIG_FILE_NAME)?;
            match config {
                ConfigCommand::Set(config_set) => {
                    match config_set {
                        ConfigSetCommand::What(config_set_what) => match config_set_what {
                            ConfigSetWhatCommand::Number(config_set_what_number) => {
                                cfg.what.number = config_set_what_number.number;
                            }
                        },
                        ConfigSetCommand::Plan(config_set_plan) => match config_set_plan {
                            ConfigSetPlanCommand::Number(config_set_plan_number) => {
                                cfg.plan.number = config_set_plan_number.number;
                            }
                            ConfigSetPlanCommand::Days(config_set_plan_days) => {
                                cfg.plan.days = config_set_plan_days.days;
                            }
                        },
                    }
                    confy::store(&app_name, CONFIG_FILE_NAME, cfg)?
                }
                ConfigCommand::Get(config_get) => match config_get {
                    ConfigGetCommand::What(config_get_what) => match config_get_what {
                        ConfigGetWhatCommand::Number(_) => {
                            println!("{}", cfg.what.number);
                        }
                    },
                    ConfigGetCommand::Plan(config_get_plan) => match config_get_plan {
                        ConfigGetPlanCommand::Number(_) => {
                            println!("{}", cfg.plan.number);
                        }
                        ConfigGetPlanCommand::Days(_) => {
                            println!("{}", cfg.plan.days);
                        }
                    },
                },
                ConfigCommand::Ignore(config_ignore) => match config_ignore {
                    ConfigIgnoreCommand::Add(config_ignore_add) => {
                        cfg.ignore.add(&config_ignore_add.meal);
                        confy::store(&app_name, CONFIG_FILE_NAME, cfg)?
                    }
                    ConfigIgnoreCommand::Remove(config_ignore_remove) => {
                        cfg.ignore.remove(&config_ignore_remove.meal);
                        confy::store(&app_name, CONFIG_FILE_NAME, cfg)?
                    }
                    ConfigIgnoreCommand::Show(_) => {
                        if !cfg.ignore.is_empty() {
                            println!("{}", cfg.ignore);
                        }
                    }
                    ConfigIgnoreCommand::Clear(_) => {
                        cfg.ignore.clear();
                        confy::store(&app_name, CONFIG_FILE_NAME, cfg)?
                    }
                },
                ConfigCommand::Path(_) => {
                    println!("{}", config_path.into_os_string().into_string()?);
                }
            }
        }
        Command::Generate(generate) => {
            let mut cmd = Cli::command();
            match generate {
                GenerateCommand::Bash(_) => {
                    print_completions(shells::Bash, &mut cmd);
                }
                GenerateCommand::Elvish(_) => {
                    print_completions(shells::Elvish, &mut cmd);
                }
                GenerateCommand::Fish(_) => {
                    print_completions(shells::Fish, &mut cmd);
                }
                GenerateCommand::Nushell(_) => {
                    print_completions(Nushell, &mut cmd);
                }
                GenerateCommand::PowerShell(_) => {
                    print_completions(shells::PowerShell, &mut cmd);
                }
                GenerateCommand::Zsh(_) => {
                    print_completions(shells::Zsh, &mut cmd);
                }
            }
        }
    };
    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut ClapCommand) {
    generate_completions(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
