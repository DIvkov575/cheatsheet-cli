use anyhow::Result;
use clap::Parser;
mod add;
mod get;
mod remove;
mod show;
mod web_sync;
mod update;
mod run;

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
        id: Vec<String>
    },
    #[command(about="get the value of an record")]
    Get {
        id: String
    },
    #[command(about="update the id of an record")]
    UpdateID {
        id: String,
        new_id: String,
    },
    #[command(about="update the key of an record")]
    UpdateKey {
        id: String,
        new_key: String,
    },
    #[command(about="update the value of an record")]
    UpdateValue {
        id: String,
        new_value: String,
    },
    #[command(about="execute the value stored in record as command")]
    Run {
        id: String
    }
}

impl Command {
    pub async fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Show => Ok(show::show_command().await?),
            InitWeb=> Ok(web_sync::init_web_sync().await?),
            Add{name, line} => Ok(add::add(name, line).await?),
            Remove{id} => Ok(remove::remove(&id).await?),
            Get{id} => Ok(get::get(&id)?),
            Run{id} => Ok(run::run(&id)?),
            UpdateID {id, new_id} => Ok(update::update_id(&id, &new_id).await?),
            UpdateKey{id, new_key} => Ok(update::update_key(&id, &new_key).await?),
            UpdateValue {id, new_value} => Ok(update::update_val(&id, &new_value).await?),
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
