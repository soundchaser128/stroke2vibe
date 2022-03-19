use std::fs;

use crate::{actions::Transformer, args::Arguments, types::Funscript};
use clap::Parser;

mod actions;
mod args;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    if let Some(level) = arguments.log {
        env_logger::builder()
            .filter_level(level)
            .default_format()
            .init();
    }

    let file = fs::read_to_string(&arguments.input)?;
    let mut funscript: Funscript = serde_json::from_str(&file)?;
    log::debug!("parsed funscript: {:#?}", funscript);
    log::info!("parsed script with {} actions", funscript.actions.len());
    let mut transformer = Transformer::new(&funscript);

    for command in arguments.commands() {
        transformer.transform(command);
    }
    funscript.actions = transformer.into_list();

    let output = if arguments.pretty {
        serde_json::to_string_pretty(&funscript)
    } else {
        serde_json::to_string(&funscript)
    }?;
    fs::write(arguments.output, &output)?;

    Ok(())
}
