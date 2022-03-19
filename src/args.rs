use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Input funscript file
    #[clap(short, long)]
    pub input: PathBuf,

    /// Destination to write to
    #[clap(short, long)]
    pub output: PathBuf,

    /// Pretty-print the output JSON
    #[clap(short, long)]
    pub pretty: bool,

    /// Enable logging (`info` or `debug`)
    #[clap(short, long)]
    pub log: Option<LevelFilter>,

    /// Command list. Allowed values:
    ///
    /// `normalize`: Normalizes the output range to 0-100
    ///
    /// `scale-linear <num>`: Multiply all output values by a given factor
    ///
    /// `scale-sqrt`: Take the square root for all output values
    ///
    /// `shorten <num>`: Removes small differences in output position values
    pub commands: Vec<String>,
}

impl Arguments {
    pub fn commands(&self) -> Vec<Command> {
        enum State {
            None,
            ScaleLinear,
            Shorten,
        }

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
