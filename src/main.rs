use std::path::PathBuf;

use anyhow::bail;
use clap::{Parser, Subcommand};

mod store;
mod task;

use task::Task;

/// A small command-line task manager.
#[derive(Parser)]
#[command(name = "todo", version, about)]
struct Cli {
    /// File the tasks are stored in.
    #[arg(long, default_value = "tasks.json")]
    file: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new task.
    Add { title: String },
    /// List all tasks.
    List,
    /// Mark a task as done.
    Done { id: u32 },
    /// Remove a task.
    Remove { id: u32 },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut tasks = store::load(&cli.file)?;

    match cli.command {
        Command::Add { title } => {
            let id = task::next_id(&tasks);
            tasks.push(Task::new(id, title));
            store::save(&cli.file, &tasks)?;
            println!("Added task {id}");
        }
        Command::List => {
            if tasks.is_empty() {
                println!("No tasks yet. Add one with: todo add \"Buy milk\"");
            }
            for t in &tasks {
                println!("{t}");
            }
        }
        Command::Done { id } => {
            if !task::mark_done(&mut tasks, id) {
                bail!("no task with id {id}");
            }
            store::save(&cli.file, &tasks)?;
            println!("Marked task {id} as done");
        }
        // TODO(#2): implement this command.
        Command::Remove { .. } => {
            bail!("not implemented yet, see the open issues")
        }
    }
    Ok(())
}
