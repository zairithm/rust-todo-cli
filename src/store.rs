use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use anyhow::Context;

use crate::task::Task;

/// Loads tasks from `path`. A missing file means no tasks yet.
pub fn load(path: &Path) -> anyhow::Result<Vec<Task>> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text)
            .with_context(|| format!("{} is not a valid task file", path.display())),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e).with_context(|| format!("could not read {}", path.display())),
    }
}

/// Saves tasks to `path` as pretty-printed JSON.
pub fn save(path: &Path, tasks: &[Task]) -> anyhow::Result<()> {
    let text = serde_json::to_string_pretty(tasks)?;
    fs::write(path, text).with_context(|| format!("could not write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let path = std::env::temp_dir().join(format!("todo-test-{}.json", std::process::id()));
        let tasks = vec![Task::new(1, "first".into())];
        save(&path, &tasks).unwrap();
        assert_eq!(load(&path).unwrap(), tasks);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn missing_file_is_empty() {
        let path = std::env::temp_dir().join("todo-test-does-not-exist.json");
        assert!(load(&path).unwrap().is_empty());
    }
}
