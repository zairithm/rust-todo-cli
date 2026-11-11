use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "x" } else { " " };
        write!(f, "[{mark}] {:>3}  {}", self.id, self.title)
    }
}

/// The id for the next task: one more than the highest id in use.
pub fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_id_starts_at_one() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn next_id_skips_past_the_highest() {
        let tasks = vec![Task::new(1, "a".into()), Task::new(5, "b".into())];
        assert_eq!(next_id(&tasks), 6);
    }

    #[test]
    fn display_shows_status() {
        let mut t = Task::new(2, "write tests".into());
        assert_eq!(t.to_string(), "[ ]   2  write tests");
        t.done = true;
        assert!(t.to_string().starts_with("[x]"));
    }
}
