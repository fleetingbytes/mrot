//! CLI for mrot

#![deny(missing_docs)]

use crate::config::MrotConfig;
use crate::error::Error;
use crate::{add_meal_on_dates, meals_between_dates, open_storage};
use clap::{ArgAction::Append, Args, Command as ClapCommand, CommandFactory, Parser, Subcommand};
use clap_complete::{generate as generate_completions, shells, Generator};
use clap_complete_nushell::Nushell;
use std::io;
use tracing::instrument;

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
}

#[derive(Args)]
struct ShowArgs {
    /// Time range to shows meals from
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
}

#[derive(Args)]
struct ConfigSetWhatNumberArgs {
    /// Max number of meals to suggest
    number: usize,
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
}

#[derive(Args)]
struct ConfigGetWhatNumberArgs;

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

const CONFIG_FILE_NAME: &str = "config";

/// Parses the CLI commands and makes the required API calls to execute them
#[instrument]
pub fn run() -> Result<(), Error> {
    let app_name = Cli::command().get_name().to_string();
    let mut cfg: MrotConfig = confy::load(&app_name, CONFIG_FILE_NAME)?;
    let cli = Cli::parse();
    match &cli.command {
        Command::Add(add) => {
            let dates = match &add.date {
                Some(vec_d) => vec_d,
                None => &vec![String::from("today")],
            };
            let storage = open_storage()?;
            add_meal_on_dates(&add.meal, &dates, &storage)?;
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
        Command::Show(show) => {
            if let Some(range) = &show.range {
                println!("show range is {}", range);
                let storage = open_storage()?;
                meals_between_dates(range, &storage)?;
            }
        }
        Command::When(when) => {
            println!("when meal is {}", when.meal);
        }
        Command::Remove(remove) => {
            println!("remove range is {}", remove.range);
            if let Some(meal) = &remove.meal {
                println!("remove meal is {}", meal);
            } else {
                println!("remove all meals in that range");
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
                        ConfigSetCommand::Show(config_set_show) => {
                            cfg.show.range = config_set_show.range.clone();
                        }
                    }
                    confy::store(&app_name, CONFIG_FILE_NAME, cfg)?
                }
                ConfigCommand::Get(config_get) => match config_get {
                    ConfigGetCommand::What(config_get_what) => match config_get_what {
                        ConfigGetWhatCommand::Number(_) => {
                            println!("{}", cfg.what.number);
                        }
                    },
                    ConfigGetCommand::Show(_) => {
                        println!("{}", cfg.show.range);
                    }
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
