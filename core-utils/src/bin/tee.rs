use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

fn should_append(args: &mut Vec<String>) -> bool {
    let mut i = 0;
    while i != args.len() {
        if args[i].eq("-a") || args[i].eq("--append") {
            args.remove(i);
            return true;
        } else {
            i += 1;
        }
    }
    false
}

struct MultiWriter {
    writers: Vec<Box<dyn Write>>
}

impl From<Vec<io::Result<File>>> for MultiWriter {
    fn from(files: Vec<io::Result<File>>) -> Self {
        let mut writers = files
            .into_iter()
            .filter_map(|file| {
                match file {
                    Ok(f) => Some(Box::new(f) as Box<dyn Write>),
                    Err(_) => None
                }
            }).collect::<Vec<Box<dyn Write>>>();
        writers.push(Box::new(io::stdout()));
        MultiWriter { writers }
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for writer in &mut self.writers {
            writer.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}

fn open_file(path: &str, append_mode: bool) -> io::Result<File> {
    let mut file = OpenOptions::new();
    if append_mode {
        file.append(true)
    } else {
        file.truncate(true)
    };
    match file.write(true).create(true).open(&path) {
        Ok(f) => Ok(f),
        Err(e) => {
            eprintln!("tee: {}: {}", path, e.to_string());
            Err(e)
        },
    }
}

fn copy(reader: &mut Box<dyn Read>, writer: &mut Box<dyn Write>, buf: &mut [u8]) -> io::Result<()> {
    while let Ok(n) = reader.read(buf) {
        if n == 0 {
            break
        }
        writer.write_all(&buf[..n])?;
        writer.flush()?;
    }
    Ok(())
}

fn tee(paths: &Vec<String>, append_mode: bool) -> io::Result<()> {
    let files = paths
        .into_iter()
        .map(|path| open_file(path, append_mode))
        .collect::<Vec<io::Result<File>>>();
    let writer = MultiWriter::from(files);
    let mut buf = [0; 32 * 1024];
    copy(
        &mut (Box::new(io::stdin()) as Box<dyn Read>),
        &mut (Box::new(writer) as Box<dyn Write>),
        &mut buf,
    )
}

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() > 0 {
        args.remove(0);
    }
    let append = should_append(&mut args);
    match tee(&args, append) {
        Err(e) => {
            eprintln!("tee: {}", e.to_string());
            std::process::exit(1);
        },
        _ => {}
    }
}
