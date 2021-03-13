use std::fs;
use std::fs::DirEntry;

fn print_file_name(file: DirEntry, show_hidden: bool) {
    match file.file_name().to_str() {
        Some(name) => {
            if show_hidden || !name.starts_with('.') {
                println!("{}", name)
            }
        }
        None => {}
    }
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => "./".into(),
    };
    let show_hidden = match std::env::args().nth(2) {
        Some(flag) if flag.eq("--show-hidden") => true,
        _ => false
    };

    let dir = fs::read_dir(path).unwrap();
    for file in dir {
        match file {
            Ok(f) => print_file_name(f, show_hidden),
            Err(_) => {}
        }
    }
}
