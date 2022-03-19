use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub output: PathBuf,

    #[clap(short, long)]
    pub pretty: bool,

    #[clap(short, long)]
    pub log: Option<LevelFilter>,

    pub commands: Vec<String>,
}

impl Arguments {
    pub fn commands(&self) -> Vec<Command> {
        let mut commands = vec![];
        let mut state = State::None;
        for token in &self.commands {
            match token.to_lowercase().as_str() {
                "normalize" => commands.push(Command::Normalize),
                "scale-linear" => state = State::ScaleLinear,
                "scale-sqrt" => commands.push(Command::ScaleSqrt),
                "shorten" => state = State::Shorten,
                string => match state {
                    State::ScaleLinear => {
                        let scale = string.parse().expect("invalid argument for scale-linear");
                        commands.push(Command::ScaleLinear { scale });
                    }
                    State::Shorten => {
                        let diff = string.parse().expect("invalid argument for shorten");
                        commands.push(Command::Shorten { diff });
                    }
                    _ => {}
                },
            }
        }
        log::info!("parsed commands: {commands:?}");

        commands
    }
}

#[derive(Debug)]
pub enum Command {
    Normalize,
    ScaleLinear { scale: f64 },
    ScaleSqrt,
    Shorten { diff: f64 },
}

enum State {
    None,
    ScaleLinear,
    Shorten,
}
