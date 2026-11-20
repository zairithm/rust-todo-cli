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

/// Marks the task with `id` as done. Returns false if no task has that id.
pub fn mark_done(tasks: &mut [Task], id: u32) -> bool {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.done = true;
            true
        }
        None => false,
    }
}

/// Removes the task with `id` and returns it. Other tasks keep their ids.
pub fn remove(tasks: &mut Vec<Task>, id: u32) -> Option<Task> {
    let index = tasks.iter().position(|t| t.id == id)?;
    Some(tasks.remove(index))
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

    #[test]
    fn mark_done_finds_the_task() {
        let mut tasks = vec![Task::new(1, "a".into()), Task::new(2, "b".into())];
        assert!(mark_done(&mut tasks, 2));
        assert!(tasks[1].done);
        assert!(!tasks[0].done);
    }

    #[test]
    fn mark_done_unknown_id() {
        let mut tasks = vec![Task::new(1, "a".into())];
        assert!(!mark_done(&mut tasks, 99));
        assert!(!tasks[0].done);
    }

    #[test]
    fn remove_keeps_other_ids() {
        let mut tasks = vec![
            Task::new(1, "a".into()),
            Task::new(2, "b".into()),
            Task::new(3, "c".into()),
        ];
        let removed = remove(&mut tasks, 2).unwrap();
        assert_eq!(removed.title, "b");
        let ids: Vec<u32> = tasks.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![1, 3]);
    }

    #[test]
    fn remove_unknown_id() {
        let mut tasks = vec![Task::new(1, "a".into())];
        assert_eq!(remove(&mut tasks, 7), None);
        assert_eq!(tasks.len(), 1);
    }
}
