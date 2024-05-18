use anyhow::Result;
use clap::Parser;
use crate::{add, remove, show, web_sync};

#[derive(Parser, Debug)]
pub enum Command {
    #[command(about="Show CheatSheet")]
    Show,
    #[command(about="Create gist to cloud save config and cheatsheet")]
    InitWeb,
    #[command(about="add a line to cheatsheet")]
    Add {
        name: String,
        line: String
    },
    #[command(about="deletes a line from cheatsheet")]
    Remove {
        id: String
    }
}

impl Command {
    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Show => show::show_command(),
            InitWeb=> web_sync::init_web_sync(),
            Add{name, line} => add::add(name, line),
            Remove{id} => remove::remove(id),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None,)]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
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
