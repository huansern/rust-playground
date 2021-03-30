use crate::error::{Error, Result};

const INCOMPLETE_TASK_PREFIX: &str = "[ ] ";
const COMPLETED_TASK_PREFIX: &str = "[x] ";
const SEPARATOR: &str = "\n";

pub struct Tasks<'a> {
    inner: Vec<Task<'a>>,
}

impl<'a> Tasks<'a> {
    pub fn new() -> Self {
        Tasks { inner: vec![] }
    }

    pub fn parse(s: &'a str) -> Result<Self> {
        let tasks = s
            .split(SEPARATOR)
            .filter(|line| !line.is_empty())
            .map(|task| Task::parse(task))
            .collect::<Result<Vec<Task>>>()?;
        Ok(Tasks { inner: tasks })
    }

    pub fn add(&mut self, description: &'a str) {
        self.inner.push(Task::new(description));
    }

    pub fn edit(&mut self, index: usize, description: &'a str) -> Result<String> {
        match self.inner.get_mut(index) {
            None => Err(Error::InvalidSequence),
            Some(task) => Ok(task.edit(description).into()),
        }
    }

    pub fn delete(&mut self, index: usize) -> Result<String> {
        if index < self.inner.len() {
            Ok(self.inner.remove(index).description.into())
        } else {
            Err(Error::InvalidSequence)
        }
    }

    pub fn complete(&mut self, index: usize) -> Result<String> {
        match self.inner.get_mut(index) {
            None => Err(Error::InvalidSequence),
            Some(task) => {
                task.complete();
                Ok(task.description.into())
            }
        }
    }

    pub fn prune(&mut self) -> usize {
        self.inner.retain(|task| !task.completed);
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn print_incomplete(&self) -> String {
        self.inner
            .iter()
            .enumerate()
            .filter(|(_, task)| !task.completed)
            .map(|(index, task)| task.sequence_format(index + 1))
            .collect::<Vec<String>>()
            .join(SEPARATOR)
    }

    pub fn persistence_format(&self) -> String {
        self.inner
            .iter()
            .map(|task| task.persistence_format())
            .collect::<Vec<String>>()
            .join(SEPARATOR)
    }
}

struct Task<'a> {
    description: &'a str,
    completed: bool,
}

impl<'a> Task<'a> {
    fn new(description: &'a str) -> Self {
        Task {
            description,
            completed: false,
        }
    }

    fn parse(s: &'a str) -> Result<Self> {
        if s.starts_with(INCOMPLETE_TASK_PREFIX) {
            Ok(Task {
                description: &s[INCOMPLETE_TASK_PREFIX.len()..],
                completed: false,
            })
        } else if s.starts_with(COMPLETED_TASK_PREFIX) {
            Ok(Task {
                description: &s[COMPLETED_TASK_PREFIX.len()..],
                completed: true,
            })
        } else {
            Err(Error::ParseTask(s.into()))
        }
    }

    fn edit(&mut self, description: &'a str) -> &str {
        let prev = self.description;
        self.description = description;
        prev
    }

    fn complete(&mut self) {
        self.completed = true
    }

    fn sequence_format(&self, sequence: usize) -> String {
        format!("{}: {}", sequence, self.description)
    }

    fn persistence_format(&self) -> String {
        if self.completed {
            format!("{}{}", COMPLETED_TASK_PREFIX, self.description)
        } else {
            format!("{}{}", INCOMPLETE_TASK_PREFIX, self.description)
        }
    }
}
