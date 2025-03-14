//! CLI for mrot

use clap::{
    ArgAction::{Append, SetTrue},
    Args, Command as ClapCommand, CommandFactory, Parser, Subcommand,
};
use clap_complete::{generate as generate_completions, shells, Generator};
use clap_complete_nushell::Nushell;
use directories::ProjectDirs;
use libmrot::{parse_date as mrot_parse, Error, Result, Storage};
use mrot_config::MrotConfig;
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

const APP_NAME: &str = "mrot";
const CONFIG_FILE_NAME: &str = "config";
const STORAGE_FILE: &str = "database.sql";

/// Parses the CLI commands and makes the required API calls to execute them.
#[instrument]
pub fn run() -> Result<()> {
    let mut cfg: MrotConfig = confy::load(APP_NAME, CONFIG_FILE_NAME)?;
    let cli = Cli::parse();
    match &cli.command {
        Command::Add(add) => {
            let dates = match &add.date {
                Some(vec_d) => vec_d,
                None => &vec![String::from("today")],
            };
            let storage = open_storage()?;
            storage.add_meal_on_dates(&add.meal, &dates)?;
        }
        Command::What(what) => {
            if let Some(ref number) = what.number {
                println!("what number is {}", number);
            }
            println!("configured number is {}", cfg.what.number);
            let number = what.number.unwrap_or(cfg.what.number);
            println!("resulting number is {}", number);
            if let Some(ref ignore) = what.ignore {
                println!("what ignore is {:?}", ignore);
            }
            println!("what no_ignore is {}", what.no_ignore);
            println!("configured ignore list is {:?}", cfg.what.ignore);
            let ignore_list = if what.no_ignore {
                Vec::new()
            } else {
                cfg.what.ignore.to_vec_string()
            };
            println!("resulting ignore list is {:?}", ignore_list);
            println!("what no_look_ahead is {}", what.no_look_ahead);
            println!("what look_ahead is {:?}", what.look_ahead);
            println!("configured look_ahead is {:?}", cfg.what.look_ahead);
            let look_ahead = if what.no_look_ahead {
                Vec::new()
            } else {
                let days = what.look_ahead.unwrap_or(cfg.what.look_ahead);
                mrot_parse(&format!(
                    "from one day after tomorrow through {} days after tomorrow",
                    days
                ))?
            };
            println!("resulting look-ahead is {:?}", look_ahead);
            println!("storage::what is run");
            let storage = open_storage()?;
            let meals = storage.what(number, &ignore_list, &look_ahead)?;
            println!("{:?}", meals);
        }
        Command::Show(show) => {
            if let Some(range) = &show.range {
                println!("show range is {}", range);
                // TODO: open actual storage on disk
                let _storage = Storage::open(":memory")?;
                println!("here I would show the meals in the given date range");
            } else {
                println!("here I would show the meals in the default date range");
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
            let config_path = confy::get_configuration_file_path(APP_NAME, CONFIG_FILE_NAME)?;
            match config {
                ConfigCommand::Set(config_set) => {
                    match config_set {
                        ConfigSetCommand::What(config_set_what) => match config_set_what {
                            ConfigSetWhatCommand::Number(config_set_what_number) => {
                                cfg.what.number = config_set_what_number.number;
                            }
                            ConfigSetWhatCommand::LookAhead(config_set_what_look_ahead) => {
                                // TODO return a proper Error;
                                assert!(config_set_what_look_ahead.look_ahead > 0, "must be >0");
                                cfg.what.look_ahead = config_set_what_look_ahead.look_ahead;
                            }
                        },
                        ConfigSetCommand::Show(config_set_show) => {
                            cfg.show.range = config_set_show.range.clone();
                        }
                    }
                    confy::store(APP_NAME, CONFIG_FILE_NAME, cfg)?
                }
                ConfigCommand::Get(config_get) => match config_get {
                    ConfigGetCommand::What(config_get_what) => match config_get_what {
                        ConfigGetWhatCommand::Number(_) => {
                            println!("{}", cfg.what.number);
                        }
                        ConfigGetWhatCommand::LookAhead(_) => {
                            println!("{}", cfg.what.look_ahead);
                        }
                    },
                    ConfigGetCommand::Show(_) => {
                        println!("{:?}", cfg.show.range);
                    }
                },
                ConfigCommand::Ignore(config_ignore) => match config_ignore {
                    ConfigIgnoreCommand::Add(config_ignore_add) => {
                        cfg.what.ignore.add(&config_ignore_add.meal);
                        confy::store(APP_NAME, CONFIG_FILE_NAME, cfg)?
                    }
                    ConfigIgnoreCommand::Remove(config_ignore_remove) => {
                        cfg.what.ignore.remove(&config_ignore_remove.meal);
                        confy::store(APP_NAME, CONFIG_FILE_NAME, cfg)?
                    }
                    ConfigIgnoreCommand::Show(_) => {
                        if !cfg.what.ignore.is_empty() {
                            println!("{}", cfg.what.ignore);
                        }
                    }
                    ConfigIgnoreCommand::Clear(_) => {
                        cfg.what.ignore.clear();
                        confy::store(APP_NAME, CONFIG_FILE_NAME, cfg)?
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
        Command::ParseDate(parse_date) => {
            let date = &parse_date.date;
            let mrot_dates = mrot_parse(&date)?;
            println!("{:?}", mrot_dates);
        }
    };
    Ok(())
}

fn open_storage() -> Result<Storage> {
    let storage_path = get_storage_path()?;
    Storage::open(&storage_path)
}

fn get_storage_path() -> Result<String> {
    let dirs = ProjectDirs::from("", "", APP_NAME)
            .ok_or(
                Error::NoDirectory(
                    "directories::ProjectDirs: no valid home directory path could be retrieved from the operating system".to_string()
                )
            )?;
    let file_path = dirs.data_dir().join(STORAGE_FILE);
    Ok(file_path.into_os_string().into_string()?)
}

fn print_completions<G: Generator>(generator: G, cmd: &mut ClapCommand) {
    generate_completions(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
