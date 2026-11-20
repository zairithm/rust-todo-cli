//! Runs the real `todo` binary against a temporary task file.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

/// A fresh task file in its own temporary directory.
struct Fixture {
    _dir: TempDir,
    file: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("tasks.json");
        Self { _dir: dir, file }
    }

    fn todo(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        cargo_bin_cmd!("todo")
            .arg("--file")
            .arg(&self.file)
            .args(args)
            .assert()
    }
}

#[test]
fn empty_list_shows_a_hint() {
    let f = Fixture::new();
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("No tasks yet"));
}

#[test]
fn add_then_list_shows_the_task() {
    let f = Fixture::new();
    f.todo(&["add", "Write tests"])
        .success()
        .stdout("Added task 1\n");
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("[ ]   1  Write tests"));
}

#[test]
fn done_marks_the_task() {
    let f = Fixture::new();
    f.todo(&["add", "Ship it"]).success();
    f.todo(&["done", "1"]).success();
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("[x]   1  Ship it"));
}

#[test]
fn remove_deletes_the_task() {
    let f = Fixture::new();
    f.todo(&["add", "keep"]).success();
    f.todo(&["add", "drop"]).success();
    f.todo(&["remove", "2"]).success();
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("keep").and(predicate::str::contains("drop").not()));
}

#[test]
fn unknown_id_fails() {
    let f = Fixture::new();
    f.todo(&["done", "42"])
        .failure()
        .stderr(predicate::str::contains("no task with id 42"));
    f.todo(&["remove", "42"])
        .failure()
        .stderr(predicate::str::contains("no task with id 42"));
}

#[test]
fn filters_by_status() {
    let f = Fixture::new();
    f.todo(&["add", "first"]).success();
    f.todo(&["add", "second"]).success();
    f.todo(&["done", "1"]).success();

    f.todo(&["list", "--pending"])
        .success()
        .stdout(predicate::str::contains("second").and(predicate::str::contains("first").not()));
    f.todo(&["list", "--done"])
        .success()
        .stdout(predicate::str::contains("first").and(predicate::str::contains("second").not()));
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("1 of 2 done"));
}

#[test]
fn pending_and_done_together_is_an_error() {
    let f = Fixture::new();
    f.todo(&["list", "--pending", "--done"]).failure();
}

#[test]
fn no_colour_when_not_a_terminal() {
    let f = Fixture::new();
    f.todo(&["add", "plain"]).success();
    f.todo(&["list"])
        .success()
        .stdout(predicate::str::contains("\u{1b}[").not());
}

#[test]
fn corrupt_file_is_reported() {
    let f = Fixture::new();
    std::fs::write(&f.file, "not json").unwrap();
    f.todo(&["list"])
        .failure()
        .stderr(predicate::str::contains("is not a valid task file"));
}
