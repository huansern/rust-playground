use std::fs;
use std::io;
use std::io::BufRead;

fn read_stdin() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => println!("{}", input),
            Err(e) => {
                return Err(e)
            }
        }
    }
    Ok(())
}

fn read_files(paths: Vec<String>) -> io::Result<()> {
    for path in paths {
        match fs::read_to_string(path) {
            Ok(data) => print!("{}", data),
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    let result = if args.is_empty() {
        read_stdin()
    } else {
        read_files(args)
    };
    match result {
        Err(e) => {
            println!("Error reading input: {}", e.to_string());
            std::process::exit(1);
        },
        _ => ()
    }
}
