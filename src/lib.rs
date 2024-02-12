pub mod cli;

use clap::Parser;
use cli::{
    Cli, Command, ConfigCommand, ConfigGetCommand, ConfigGetPlanCommand, ConfigGetWhatCommand,
    ConfigIgnoreCommand, ConfigSetCommand, ConfigSetPlanCommand, ConfigSetWhatCommand,
    GenerateCommand, PlanCommand, PlanRemoveCommand,
};

pub fn translate_cli_to_api() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Add(add) => {
            println!("meal is {}", add.meal);
            if let Some(ref date) = add.date {
                println!("meal date is {}", date);
            }
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
        Command::Plan(plan) => match plan {
            PlanCommand::Add(plan_add) => {
                println!("plan add meal {} on date {}", plan_add.meal, plan_add.date);
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
        },
        Command::Random(_) => {
            println!("random is run");
        }
        Command::Config(config) => match config {
            ConfigCommand::Set(config_set) => match config_set {
                ConfigSetCommand::What(config_set_what) => match config_set_what {
                    ConfigSetWhatCommand::Number(config_set_what_number) => {
                        println!(
                            "config set what number is {}",
                            config_set_what_number.number
                        );
                    }
                },
                ConfigSetCommand::Plan(config_set_plan) => match config_set_plan {
                    ConfigSetPlanCommand::Number(config_set_plan_number) => {
                        println!(
                            "config set plan number is {}",
                            config_set_plan_number.number
                        );
                    }
                    ConfigSetPlanCommand::Days(config_set_plan_days) => {
                        println!("config set plan days is {}", config_set_plan_days.days);
                    }
                },
            },
            ConfigCommand::Get(config_get) => match config_get {
                ConfigGetCommand::What(config_get_what) => match config_get_what {
                    ConfigGetWhatCommand::Number(_) => {
                        println!("config get what number is run");
                    }
                },
                ConfigGetCommand::Plan(config_get_plan) => match config_get_plan {
                    ConfigGetPlanCommand::Number(_) => {
                        println!("config get plan number is run");
                    }
                    ConfigGetPlanCommand::Days(_) => {
                        println!("config get plan days is run");
                    }
                },
            },
            ConfigCommand::Ignore(config_ignore) => match config_ignore {
                ConfigIgnoreCommand::Add(config_ignore_add) => {
                    println!("config ignore add is {}", config_ignore_add.meal);
                }
                ConfigIgnoreCommand::Remove(config_ignore_remove) => {
                    println!("config ignore remove is {}", config_ignore_remove.meal);
                }
                ConfigIgnoreCommand::Show(_) => {
                    println!("config ignore show is run");
                }
                ConfigIgnoreCommand::Clear(_) => {
                    println!("config ignore clear is run");
                }
            },
            ConfigCommand::Path(_) => {
                println!("config path is run");
            }
        },
        Command::Generate(generate) => match generate {
            GenerateCommand::Bash(_) => {
                println!("generate bash is run");
            }
            GenerateCommand::Elvish(_) => {
                println!("generate elvish is run");
            }
            GenerateCommand::Fish(_) => {
                println!("generate fish is run");
            }
            GenerateCommand::PowerShell(_) => {
                println!("generate powershell is run");
            }
            GenerateCommand::Zsh(_) => {
                println!("generate zsh is run");
            }
        },
    }
}
