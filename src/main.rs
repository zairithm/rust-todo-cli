use std::path::PathBuf;

use anyhow::bail;
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize, Stream};

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
    /// List tasks.
    List {
        /// Show only tasks that are not done yet.
        #[arg(long, conflicts_with = "done")]
        pending: bool,
        /// Show only tasks that are done.
        #[arg(long)]
        done: bool,
    },
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
        Command::List { pending, done } => {
            if tasks.is_empty() {
                println!("No tasks yet. Add one with: todo add \"Buy milk\"");
                return Ok(());
            }
            let shown = tasks
                .iter()
                .filter(|t| (!pending || !t.done) && (!done || t.done));
            for t in shown {
                // Colours only when printing to a terminal that supports them (and not with NO_COLOR).
                if t.done {
                    println!("{}", t.if_supports_color(Stream::Stdout, |t| t.dimmed()));
                } else {
                    println!("{}", t.if_supports_color(Stream::Stdout, |t| t.bold()));
                }
            }
            let finished = tasks.iter().filter(|t| t.done).count();
            println!("{finished} of {} done", tasks.len());
        }
        Command::Done { id } => {
            if !task::mark_done(&mut tasks, id) {
                bail!("no task with id {id}");
            }
            store::save(&cli.file, &tasks)?;
            println!("Marked task {id} as done");
        }
        Command::Remove { id } => {
            let Some(removed) = task::remove(&mut tasks, id) else {
                bail!("no task with id {id}");
            };
            store::save(&cli.file, &tasks)?;
            println!("Removed task {id}: {}", removed.title);
        }
    }
    Ok(())
}
