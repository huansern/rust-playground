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

#[cfg(test)]
pub mod tests {
    use crate::task::{Task, Tasks, COMPLETED_TASK_PREFIX, INCOMPLETE_TASK_PREFIX};

    const FIRST_TASK: &str = "First thing to do.";
    const SECOND_TASK: &str = "Second thing to do.";

    #[test]
    pub fn task_parse_empty_string_should_return_error() {
        let input = "";
        let task = Task::parse(input);
        assert!(
            task.is_err(),
            "Task parse empty string should return an error."
        );
    }

    #[test]
    pub fn task_parse_invalid_format_should_return_error() {
        let input = "A random sentence.";
        let task = Task::parse(input);
        assert!(
            task.is_err(),
            "Task parse an invalid input format should return an error."
        );
    }

    #[test]
    pub fn task_parse_incomplete_task() {
        let input = format!("{}{}", INCOMPLETE_TASK_PREFIX, FIRST_TASK);
        let task = Task::parse(&input).unwrap();
        assert!(
            !task.completed,
            "Task parse an incomplete task string returned a completed task."
        );
        assert_eq!(
            task.description, FIRST_TASK,
            "Task parse an incomplete task string returned incorrect description."
        );
    }

    #[test]
    pub fn task_parse_completed_task() {
        let input = format!("{}{}", COMPLETED_TASK_PREFIX, FIRST_TASK);
        let task = Task::parse(&input).unwrap();
        assert!(
            task.completed,
            "Task parse a completed task string returned an incomplete task."
        );
        assert_eq!(
            task.description, FIRST_TASK,
            "Task parse a completed task string returned incorrect description."
        );
    }

    #[test]
    pub fn task_edit_should_update_title() {
        let mut task = Task::new(FIRST_TASK);
        let previous: String = task.edit(SECOND_TASK).into();
        assert_eq!(
            task.description, SECOND_TASK,
            "Task edit method failed to update task title."
        );
        assert_eq!(
            previous, FIRST_TASK,
            "Task edit method should return the previous description."
        )
    }

    #[test]
    pub fn task_persistence_format_incomplete_task() {
        let task = Task::new(FIRST_TASK);
        assert_eq!(
            task.persistence_format(),
            format!("{}{}", INCOMPLETE_TASK_PREFIX, FIRST_TASK)
        );
    }

    #[test]
    pub fn task_persistence_format_completed_task() {
        let mut task = Task::new(FIRST_TASK);
        task.complete();
        assert_eq!(
            task.persistence_format(),
            format!("{}{}", COMPLETED_TASK_PREFIX, FIRST_TASK)
        );
    }

    const SOME_RANDOM_TASKS: &str =
        "[ ] First thing to do.\n[x] Some random stuff.\n[ ] More things to do.";

    #[test]
    pub fn tasks_parse_empty_string() {
        let input = "";
        let tasks = Tasks::parse(input).unwrap();
        assert!(tasks.inner.is_empty());
    }

    #[test]
    pub fn tasks_parse_input_with_empty_line() {
        let input = format!(
            "{}{}\n\n{}{}",
            COMPLETED_TASK_PREFIX, FIRST_TASK, INCOMPLETE_TASK_PREFIX, SECOND_TASK
        );
        let tasks = Tasks::parse(&input).unwrap();
        assert_eq!(tasks.inner.len(), 2);
    }

    #[test]
    pub fn tasks_parse_input() {
        let input = SOME_RANDOM_TASKS;
        let tasks = Tasks::parse(input).unwrap();
        assert_eq!(tasks.inner.len(), 3);
    }

    #[test]
    pub fn tasks_add_should_add_new_task() {
        let mut tasks = Tasks::new();
        tasks.add(FIRST_TASK);
        assert_eq!(
            tasks.inner.len(),
            1,
            "Tasks add method failed to add new task."
        );
    }

    #[test]
    pub fn tasks_edit_should_update_task_description() {
        let mut tasks = Tasks::parse(SOME_RANDOM_TASKS).unwrap();
        tasks.edit(0, FIRST_TASK).unwrap();
        let task = tasks.inner.get(0).unwrap();
        assert_eq!(
            task.description, FIRST_TASK,
            "Tasks edit method failed to update task title."
        );
    }

    #[test]
    pub fn tasks_delete_should_remove_task() {
        let mut tasks = Tasks::parse(SOME_RANDOM_TASKS).unwrap();
        tasks.delete(0).unwrap();
        assert_eq!(
            tasks.inner.len(),
            2,
            "Tasks delete method failed to remove task."
        );
    }

    #[test]
    pub fn tasks_complete_should_set_task_as_completed() {
        let mut tasks = Tasks::new();
        tasks.add(FIRST_TASK);
        tasks.complete(0).unwrap();
        let task = tasks.inner.get(0).unwrap();
        assert!(
            task.completed,
            "Tasks complete method failed to set task as completed."
        );
    }

    #[test]
    pub fn tasks_prune_should_remove_completed_tasks() {
        let mut tasks = Tasks::parse(SOME_RANDOM_TASKS).unwrap();
        tasks.prune();
        assert_eq!(
            tasks.inner.len(),
            2,
            "Tasks prune method failed to remove completed tasks."
        );
    }

    #[test]
    pub fn tasks_persistence_format() {
        let mut task_list = Tasks::new();
        task_list.add(FIRST_TASK);
        task_list.add(SECOND_TASK);
        task_list.complete(0).unwrap();
        assert_eq!(
            task_list.persistence_format(),
            format!(
                "{}{}\n{}{}",
                COMPLETED_TASK_PREFIX, FIRST_TASK, INCOMPLETE_TASK_PREFIX, SECOND_TASK
            ),
            "Tasks persistence format method returned incorrect string."
        );
    }
}
