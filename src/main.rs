use anyhow::anyhow;
use structopt::StructOpt;
use std::path::PathBuf;

mod cli;
mod tasks;

use tasks::Task;
use cli::{Action::*, CommandLineArgs};


fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {      // map only takes action if value is Some
        path.push(".rusty-journal.json");
        path
    })
}


fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file) // calls the function if value is None
        .ok_or(anyhow!("Failed to find the journal file."))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
