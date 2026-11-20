# rust-todo-cli

[![CI](https://github.com/zairithm/rust-todo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/zairithm/rust-todo-cli/actions/workflows/ci.yml)

A small command-line task manager written in Rust. Tasks are stored in a JSON file.

## Demo

```console
$ todo add "Read chapter 8 of the Rust book"
Added task 1
$ todo add "Finish the kv-store CLI"
Added task 2
$ todo add "Review a pull request"
Added task 3
$ todo done 1
Marked task 1 as done
$ todo list
[x]   1  Read chapter 8 of the Rust book
[ ]   2  Finish the kv-store CLI
[ ]   3  Review a pull request
1 of 3 done
$ todo list --pending
[ ]   2  Finish the kv-store CLI
[ ]   3  Review a pull request
1 of 3 done
$ todo remove 3
Removed task 3: Review a pull request
$ todo done 9
Error: no task with id 9
```

On a terminal, pending tasks are bold and finished ones are dimmed. Piped output and `NO_COLOR=1` stay plain.

## Install

With Rust installed ([rustup.rs](https://rustup.rs)):

```bash
cargo install --git https://github.com/zairithm/rust-todo-cli
```

Or from a clone:

```bash
git clone https://github.com/zairithm/rust-todo-cli
cd rust-todo-cli
cargo install --path .
```

This puts a `todo` binary in `~/.cargo/bin`.

## Usage

| Command | What it does |
|---|---|
| `todo add <title>` | Add a task |
| `todo list` | List all tasks with a done count |
| `todo list --pending` / `--done` | Only unfinished / finished tasks |
| `todo done <id>` | Mark a task as done |
| `todo remove <id>` | Delete a task (other ids stay the same) |
| `todo --file <path> ...` | Use another task file (default `tasks.json` in the current folder) |

## How it is built

```
src/
  main.rs   Command-line parsing with clap derive, and the command handlers
  task.rs   The Task type and the logic: next_id, mark_done, remove
  store.rs  Loading and saving the JSON file
tests/
  cli.rs    Runs the real binary against a temporary file
```

- `clap` derive for subcommands and flags, with `--pending` and `--done` declared as conflicting
- `serde` and `serde_json` for the task file; a missing file means an empty list
- `anyhow` for errors with context, such as which file could not be read
- Logic lives in `task.rs` as plain functions so it can be unit tested without touching files
- Integration tests use `assert_cmd`, `predicates` and `tempfile`

## Development

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

CI runs all three on every push and pull request.

## Roadmap

- [x] `add` and `list` commands with JSON storage
- [x] `done` command
- [x] `remove` command
- [x] Integration tests for the CLI
- [x] Coloured output and `--pending` / `--done` filters
- [x] Install instructions and a demo
- [ ] Due dates and sorting by date
- [ ] Edit a task's title
- [ ] Store the task file in the user's data folder by default

## License

MIT
