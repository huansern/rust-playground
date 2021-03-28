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
            .map(|task| Task::parse(task))
            .collect::<Result<Vec<Task>>>()?;
        Ok(Tasks { inner: tasks })
    }

    pub fn add(&mut self, description: &'a str) {
        self.inner.push(Task::new(description));
    }

    pub fn edit(&mut self, index: usize, description: &'a str) -> Result<()> {
        match self.inner.get_mut(index) {
            None => Err(Error::InvalidSequence),
            Some(task) => Ok(task.edit(description)),
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

    pub fn print(&self) -> String {
        self.inner
            .iter()
            .filter(|task| !task.completed)
            .enumerate()
            .map(|(index, task)| task.print(index + 1))
            .collect::<Vec<String>>()
            .join(SEPARATOR)
    }
}

impl<'a> ToString for Tasks<'a> {
    fn to_string(&self) -> String {
        self.inner
            .iter()
            .map(|task| task.to_string())
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

    fn edit(&mut self, description: &'a str) {
        self.description = description
    }

    fn complete(&mut self) {
        self.completed = true
    }

    fn print(&self, sequence: usize) -> String {
        format!("{}: {}", sequence, self.description)
    }
}

impl<'a> ToString for Task<'a> {
    fn to_string(&self) -> String {
        if self.completed {
            format!("{}{}", COMPLETED_TASK_PREFIX, self.description)
        } else {
            format!("{}{}", INCOMPLETE_TASK_PREFIX, self.description)
        }
    }
}
