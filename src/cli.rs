use anyhow::Result;
use clap::Parser;
use crate::{add, remove, show, web_sync, get};

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
    },
    #[command(about="get the value of an entry")]
    Get {
        id: String
    },
}

impl Command {
    pub async fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Show => Ok(show::show_command().await?),
            InitWeb=> Ok(web_sync::init_web_sync().await?),
            Add{name, line} => Ok(add::add(name, line).await?),
            Remove{id} => Ok(remove::remove(id).await?),
            Get{id} => Ok(get::get(&id)?),
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
    pub async fn run(self) -> Result<()> {
        match self.command {
            None => {Command::Show.run().await?},
            Some(command) => {command.run().await?},
        }
        Ok(())
    }
}
