use std::io::{Error, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::{fs, io};

fn read_dir(path: &Path) -> io::Result<Vec<String>> {
    Ok(path
        .read_dir()?
        .filter_map(|file| match file.ok()?.file_name().to_str() {
            Some(name) => Some(name.to_owned()),
            None => None,
        })
        .collect::<Vec<String>>())
}

pub struct App {
    working_directory: PathBuf,
}

impl App {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        match fs::metadata(&path) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            Err(e) => return Err(e),
        }
        Ok(App {
            working_directory: path,
        })
    }

    pub fn get_workdir(&self) -> PathBuf {
        PathBuf::from(&self.working_directory)
    }

    pub fn get_workdir_items(&self) -> Result<Vec<String>, String> {
        match read_dir(&*self.working_directory) {
            Err(e) => Err(e.to_string()),
            Ok(items) => Ok(items),
        }
    }

    pub fn get_parent_dir_name(&self) -> &str {
        match self.working_directory.components().last() {
            None => "",
            Some(name) => name.as_os_str().to_str().unwrap_or(""),
        }
    }

    pub fn get_parent_dir_items(&self) -> Result<Vec<String>, String> {
        match self.working_directory.parent() {
            None => Ok(Vec::new()),
            Some(dir) => match read_dir(dir) {
                Err(e) => Err(e.to_string()),
                Ok(items) => Ok(items),
            },
        }
    }

    pub fn get_child_dir_items(&self, dir_name: &str) -> Result<Vec<String>, String> {
        if dir_name.is_empty() {
            return Ok(Vec::new());
        }
        let mut workdir = PathBuf::from(&self.working_directory);
        workdir.push(dir_name);
        match workdir.canonicalize() {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        if workdir.is_file() {
            return Ok(Vec::new());
        }
        match read_dir(&*workdir) {
            Err(e) => Err(e.to_string()),
            Ok(items) => Ok(items),
        }
    }

    pub fn select_parent_dir(&mut self) -> String {
        let workdir = PathBuf::from(&self.working_directory);
        match workdir.components().last() {
            None => "".into(),
            Some(Component::RootDir) => "".into(),
            Some(c) => {
                match self.working_directory.parent() {
                    None => {}
                    Some(path) => {
                        self.working_directory = PathBuf::from(path);
                    }
                }
                c.as_os_str().to_str().unwrap_or("").into()
            }
        }
    }

    pub fn select_child_dir(&mut self, dir_name: &str) -> bool {
        if dir_name.is_empty() {
            return false;
        }
        let mut workdir = PathBuf::from(&self.working_directory);
        workdir.push(dir_name);
        match workdir.canonicalize() {
            Err(_) => false,
            Ok(path) => {
                if path.is_file() {
                    return false;
                }
                self.working_directory = PathBuf::from(path);
                true
            }
        }
    }
}
