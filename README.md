# rust-todo-cli

[![CI](https://github.com/zairithm/rust-todo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/zairithm/rust-todo-cli/actions/workflows/ci.yml)

A command-line task manager. Tasks are stored in a JSON file, and the CLI is built with `clap`.

## What this project covers

- Argument parsing with `clap` derive
- Serialisation with `serde` and `serde_json`
- Error handling with `anyhow` and `Result`
- Unit tests for the core logic

## Getting started

```bash
cargo run -- add "Write the README"
cargo run -- list
```

## Roadmap

Each open item is tracked as a [GitHub issue](https://github.com/zairithm/rust-todo-cli/issues).

- [x] `add` and `list` commands with JSON storage
- [ ] `done` command
- [ ] `remove` command
- [ ] Integration tests for the CLI
- [ ] Coloured output and a `--all/--pending` filter
- [ ] Install with `cargo install --path .` and a demo in the README

## License

MIT
