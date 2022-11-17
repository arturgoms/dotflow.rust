use clap::{Command, Arg, ArgAction};
use std::fs;
use home::home_dir;
use std::path::Path;
use serde_derive::Deserialize;
use std::process::exit;
use toml;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Default)]
struct Config {
    #[serde(default = "default_config")]
    path: String,
}

fn default_config() -> String {
    let home = match home_dir() {
        Some(path) => path.display().to_string(),
        None =>{
            eprintln!("Unable to get home dir.");
            exit(1)
        } 
    };

    return format!("{home}/.config");
}

fn check_initialization() -> Result<Data, String>{
    let home = match home_dir() {
        Some(path) => path.display().to_string(),
        None => {
            eprintln!("Could not get home path");
            exit(1);
        },
    };

    let config_path: &String = &format!("{home}/.dotflow.toml");
    if !Path::new(&config_path).exists() {
        _ = fs::File::create(config_path);
    }

    let contents = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", config_path);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", config_path);
            exit(1);
        }
    };

    Ok(data)
}

fn link(path: Option<&String>, data: Data) -> Result<bool, String> {
    println!("'dotflow link' was used, name is: {:?}", path);
    println!("{}", data.config.path);
    Ok(true)
}

fn main() {
    let matches = Command::new("Dotflow")
        .version("1.0")
        .author("Artur <contact@arturgomes.com>")
        .about("A manager for your dotfiles")
        .subcommand(
            Command::new("link")
                .about("Link files to dotflow")
                .arg(Arg::new("PATH").short('p').long("path").required(true).action(ArgAction::Set)),
        )
        .get_matches();

    let data: Data = match check_initialization() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Unable to initialize dotflow");
            exit(1)
        }
    };

    match matches.subcommand() {
        Some(("link", sub_matches)) => {
            match link(sub_matches.get_one::<String>("PATH"), data) {
                Ok(_) => true,
                Err(_) => false
            };
        }, 
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
