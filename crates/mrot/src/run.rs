use crate::{cli::*, Error, Result, LOG_FILE, PKG_NAME};
use clap::{Command as ClapCommand, CommandFactory, Parser};
use clap_complete::{generate as generate_completions, shells, Generator};
use clap_complete_nushell::Nushell;
use directories::ProjectDirs;
use libmrot::{convert_to_timestamps, parse_date as mrot_parse, LookAhead, Storage};
use mrot_config::MrotConfig;
use std::io;
use tracing::instrument;

const APP_NAME: &str = PKG_NAME;
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
            let ignore_list: Vec<String> = if what.no_ignore {
                Vec::new()
            } else {
                match what.ignore {
                    None => cfg.what.ignore.to_vec_string(),
                    Some(ref vec) => vec.clone(),
                }
            };
            println!("resulting ignore list is {:?}", ignore_list);
            println!("what no_look_ahead is {}", what.no_look_ahead);
            println!("what look_ahead is {:?}", what.look_ahead);
            println!("configured look_ahead is {:?}", cfg.what.look_ahead);
            let option_look_ahead: Option<LookAhead> = match what.no_look_ahead {
                // user explicitly used --no-look-ahead, overriding the Option<String> from the
                // config with None.
                // This will result in the None variant of Option<LookAhead>, i.e. no look-ahead.
                true => LookAhead::new(None)?,
                // there may be a look-ahead
                false => match what.look_ahead {
                    // the cli option --look-ahead was not explicitly used, so we use what is in
                    // the config.
                    // The cfg.what.look_ahead value is an Option<String>.
                    // LookAhead::new(cfg.what.look_ahead)? will be an Option<LookAhead>.
                    // If the config contains the None variant of Option<String>,
                    // the result will be the None variant of Option<LookAhead>, i. e. no look-ahead.
                    // If the config contains a Some vairant of Option<String>,
                    // the result will be the Some variant of Option<LookAhead>, i. e. some look-ahead.
                    None => LookAhead::new(cfg.what.look_ahead)?,
                    // the cli option --look-ahead was explicitly used. The user wants to override
                    // the Option<String> value from the config.
                    // Here he only has the possibility to override it with a Some variant.
                    // If he wished to override the config value with a None variant,
                    // he should have done it by using the --no-look-ahead cli option
                    Some(ref date) => LookAhead::new(Some(date.clone()))?,
                },
            };
            println!("resulting look-ahead is {:?}", option_look_ahead);
            println!("storage::what is run");
            let storage = open_storage()?;
            let meals = storage.what(number, option_look_ahead, ignore_list)?;
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

        Command::Random(_) => {
            println!("random is run");
        }

        Command::Remove(remove) => {
            println!("remove range is {}", remove.range);
            if let Some(meal) = &remove.meal {
                println!("remove meal is {}", meal);
            } else {
                println!("remove all meals in that range");
            }
        }

        Command::Config(config) => match config {
            ConfigCommand::Set(config_set) => {
                match config_set {
                    ConfigSetCommand::What(config_set_what) => match config_set_what {
                        ConfigSetWhatCommand::Number(config_set_what_number) => {
                            cfg.what.number = config_set_what_number.number;
                        }
                        ConfigSetWhatCommand::LookAhead(config_set_what_look_ahead) => {
                            verify_look_ahead_value(config_set_what_look_ahead.look_ahead.clone())?;
                            cfg.what.look_ahead = config_set_what_look_ahead.look_ahead.clone();
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
                        println!("{:?}", cfg.what.look_ahead);
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
        },

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
            match &parse_date.output_timestamp {
                false => {
                    let mrot_dates = mrot_parse(&date)?;
                    println!("{:?}", mrot_dates);
                }
                true => {
                    let date_vec = vec![String::from(date)];
                    let converted_dates: Vec<i64> = convert_to_timestamps(&date_vec)?;
                    println!("{:?}", converted_dates);
                }
            };
        }

        Command::Path(path) => match path {
            PathCommand::Config(_) => {
                let config_path = get_config_path()?;
                println!("{}", config_path);
            }
            PathCommand::Records(_) => {
                let storage_path = get_storage_path()?;
                println!("{}", storage_path);
            }
            PathCommand::Log(_) => {
                let log_path = get_log_path()?;
                println!("{}", log_path);
            }
        },
    };
    Ok(())
}

fn open_storage() -> Result<Storage> {
    let storage_path = get_storage_path()?;
    Ok(Storage::open(&storage_path)?)
}

fn get_data_file_path(file: &str) -> Result<String> {
    let dirs = ProjectDirs::from("", "", APP_NAME)
            .ok_or(
                Error::NoDirectory(
                    "directories::ProjectDirs: no valid home directory path could be retrieved from the operating system".to_string()
                )
            )?;
    let file_path = dirs.data_dir().join(file);
    Ok(file_path.into_os_string().into_string()?)
}

fn get_config_path() -> Result<String> {
    let config_path = confy::get_configuration_file_path(APP_NAME, CONFIG_FILE_NAME)?;
    Ok(config_path.into_os_string().into_string()?)
}

fn get_storage_path() -> Result<String> {
    get_data_file_path(STORAGE_FILE)
}

fn get_log_path() -> Result<String> {
    get_data_file_path(LOG_FILE)
}

fn print_completions<G: Generator>(generator: G, cmd: &mut ClapCommand) {
    generate_completions(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn verify_look_ahead_value(value: Option<String>) -> Result<()> {
    match value {
        None => Ok(()),
        Some(ref date_expression) => Ok(mrot_parse(date_expression).map(|_| ())?),
    }
}
