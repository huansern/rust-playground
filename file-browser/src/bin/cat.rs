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

fn read_file(path: &str) -> io::Result<()> {
    match fs::read_to_string(path) {
        Ok(data) => {
            print!("{}", data);
            Ok(())
        }
        Err(e) => Err(e)
    }
}

fn main() {
    let result = match std::env::args().nth(1) {
        Some(input) => read_file(&input),
        None => read_stdin()
    };
    match result {
        Err(e) => {
            println!("Error reading input: {}", e.to_string());
            std::process::exit(1);
        },
        _ => ()
    }
}
