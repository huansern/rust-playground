use crate::error::Result;
use crate::task::Tasks;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
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

pub fn print(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let tasks = Tasks::parse(&content)?;
    println!("{}", tasks.print_incomplete());
    Ok(())
}

pub fn add(path: &str, description: &str) -> Result<()> {
    let mut file = create_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.add(description);
    write_content(&mut file, &tasks)?;
    Ok(())
}

pub fn edit(path: &str, index: usize, description: &str) -> Result<()> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.edit(index, description)?;
    write_content(&mut file, &tasks)?;
    Ok(())
}

pub fn delete(path: &str, index: usize) -> Result<()> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.delete(index)?;
    write_content(&mut file, &tasks)?;
    Ok(())
}

pub fn complete(path: &str, index: usize) -> Result<()> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.complete(index)?;
    write_content(&mut file, &tasks)?;
    Ok(())
}

pub fn prune(path: &str) -> Result<()> {
    let mut file = open_file(path)?;
    let content = read_content(&mut file)?;
    let mut tasks = Tasks::parse(&content)?;
    tasks.prune();
    if tasks.is_empty() {
        drop(file);
        fs::remove_file(path)?;
    } else {
        write_content(&mut file, &tasks)?;
    }
    Ok(())
}
