use std::fs;
use serde::{Serialize, Deserialize};
use clap::{arg, command, Arg, Command};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum BuildLanguage {
    Go,
    Nix,
    Rust,
    Typescript,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum BuildTemplate {
    Docker,
    Pulumi,
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildConfiguration {
    name: String,
    language: BuildLanguage,
    template: BuildTemplate,
}

fn main() {
    let cmd = command!()
        .arg(Arg::new("name"))
        .subcommand(
            Command::new("generate")
                .about("generates a build template from a config file")
                .arg(arg!(["config"]))
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("generate", generate_matches)) => {
            let filepath = generate_matches.get_one::<String>("config");
            let contents = fs::read_to_string(filepath.unwrap()).expect("Should have been able to read config.");

            let config: BuildConfiguration = serde_yaml::from_str(&contents).expect("should have been able to unwrap config");

            println!("{}", serde_json::to_string(&config).unwrap())
        },

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
