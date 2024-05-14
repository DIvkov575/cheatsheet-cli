mod add;
mod show;

use std::fs::{File, read_to_string};
use std::io::{Error, ErrorKind};
use anyhow::{Result};
use dirs::home_dir;
use clap::Parser;

fn main() -> Result<()> {
    Args::parse().run()?;

    Ok(())
}


#[derive(Parser, Debug)]
enum Command {
    #[command(about="Show CheatSheet")]
    Show,
    #[command(about="add a line to cheatsheet")]
    Add {
        #[arg(short, long)]
        line: String
    },

}

impl Command {
    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Show => show::show(),
            Add{line} => add::add(line),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None,)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

impl Args {
    pub fn run(self) -> Result<()> {
        match self.command {
            None => {Command::Show.run()?},
            Some(command) => command.run()?,
        }
        Ok(())
    }
}


#[derive(thiserror::Error, Debug)]
enum CS_Error {
    #[error("please ensure $HOME environment variable is set")]
    MissingHomeDir
}