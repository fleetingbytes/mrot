use crate::cli::*;
use clap::{Command as ClapCommand, CommandFactory, Parser};
use clap_complete::{generate as generate_completions, shells, Generator};
use clap_complete_nushell::Nushell;
use directories::ProjectDirs;
use libmrot::{parse_date as mrot_parse, Error, Result, Storage};
use mrot_config::MrotConfig;
use std::io;
use tracing::instrument;

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
