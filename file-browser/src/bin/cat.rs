use std::{fs, io};
use std::io::{Read, Write};

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

fn read_stdin(writer: &mut Box<dyn Write>, buf: &mut [u8]) -> io::Result<()> {
    copy(&mut (Box::new(io::stdin()) as Box<dyn Read>), writer, buf)
}

fn read_files(paths: Vec<String>, writer: &mut Box<dyn Write>, buf: &mut [u8]) -> io::Result<()> {
    for path in paths {
        match fs::File::open(path) {
            Ok(file) => copy(&mut (Box::new(file) as Box<dyn Read>), writer, buf)?,
            Err(e) => return Err(e)
        }
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    let mut stdout = Box::new(io::stdout()) as Box<dyn Write>;
    let mut buf = [0; 32 * 1024];
    let result = if args.is_empty() {
        read_stdin(&mut stdout, &mut buf)
    } else {
        read_files(args, &mut stdout, &mut buf)
    };
    match result {
        Err(e) => {
            println!("Error reading input: {}", e.to_string());
            std::process::exit(1);
        },
        _ => ()
    }
}
