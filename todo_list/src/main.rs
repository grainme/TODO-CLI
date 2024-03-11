use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod task;

use cli::{Action::*, CommandLineArgs};

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".tasks.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command line arguments
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_path = match find_default_journal_file() {
        Some(path) => path,
        None => journal_file.expect("Failed to find the journal file"),
    };

    match action {
        Add { task } => task::add_task(journal_path, task::Task::new(task)),
        Done { position } => task::complete(journal_path, position),
        List => task::list_tasks(journal_path),
    }?;

    Ok(())
}
