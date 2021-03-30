use crate::error::Result;
use crate::task::Tasks;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::{fs, io};

fn get_file_size(file: &File) -> usize {
    file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
}

fn read_content(file: &mut File) -> io::Result<String> {
    let mut content = String::with_capacity(get_file_size(file));
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_content(file: &mut File, tasks: &Tasks) -> io::Result<()> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    let content = tasks.persistence_format();
    file.write_all(content.as_ref())
}

fn create_file(path: &str) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

fn open_file(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

pub fn print(path: &str) -> Result<String> {
    let content = match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok("".into()),
        Err(err) => Err(err),
    }?;
    let tasks = Tasks::parse(&content)?;
    Ok(tasks.print_incomplete())
}

pub fn add(path: &str, description: &str) -> Result<String> {
    let mut file = create_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.add(description);
    write_content(&mut file, &tasks)?;
    Ok(format!("New to-do: {}", description))
}

pub fn edit(path: &str, index: usize, description: &str) -> Result<String> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    let prev = tasks.edit(index, description)?;
    write_content(&mut file, &tasks)?;
    Ok(format!("{} → {}", prev, description))
}

pub fn delete(path: &str, index: usize) -> Result<String> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    let task = tasks.delete(index)?;
    write_content(&mut file, &tasks)?;
    Ok(format!("Delete to-do: {}", task))
}

pub fn complete(path: &str, index: usize) -> Result<String> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    let task = tasks.complete(index)?;
    write_content(&mut file, &tasks)?;
    Ok(format!("✔ {}", task))
}

pub fn prune(path: &str) -> Result<String> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    if tasks.is_empty() {
        return Ok("".into());
    }
    let before = tasks.len();
    let after = tasks.prune();
    if tasks.is_empty() {
        drop(file);
        fs::remove_file(path)?;
        Ok(format!(
            "Removed {} completed to-do(s), no to-do left.",
            before
        ))
    } else {
        write_content(&mut file, &tasks)?;
        Ok(format!(
            "Removed {} completed to-do(s), {} to-do(s) remaining.",
            before - after,
            after
        ))
    }
}
