mod app;
mod error;
mod task;

use crate::error::{Error, Result};
use clap::{clap_app, ArgMatches};
use std::env;
use std::path::PathBuf;

fn main() {
    let args = clap_app!(app =>
        (name: "Quest")
        (version: "0.1.0")
        (about: "Quest is a simple cli to-do app.")
        (@arg FILE: -f --file [FILE] "Specify an alternate to-do file\n(the default is quest file in current directory)")
        (@arg EDIT: -e --edit [SEQUENCE] "Edit task description")
        (@arg DELETE: -d --delete [SEQUENCE] "Delete a task")
        (@arg COMPLETE: -c --complete [SEQUENCE] "Complete a task")
        (@arg PRUNE: -p --prune "Delete completed tasks")
        (@arg TASK: "Task description")
    )
        .get_matches();

    let input_file = match args.value_of("FILE") {
        None => env::current_dir().unwrap().join("quest"),
        Some(path) => PathBuf::from(path),
    };

    let path = input_file.as_os_str().to_str().unwrap();

    let r = run(&args, path);
    print(r);
}

fn print(r: Result<String>) {
    match r {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(s) if s.len() > 0 => println!("{}", s),
        Ok(_) => {}
    }
}

fn run(args: &ArgMatches, path: &str) -> Result<String> {
    if args.is_present("EDIT") {
        let index = parse_sequence(args.value_of("EDIT").unwrap())?;
        let description = get_description(args)?;
        app::edit(path, index, &description)
    } else if args.is_present("DELETE") {
        let index = parse_sequence(args.value_of("DELETE").unwrap())?;
        app::delete(path, index)
    } else if args.is_present("COMPLETE") {
        let index = parse_sequence(args.value_of("COMPLETE").unwrap())?;
        app::complete(path, index)
    } else if args.is_present("PRUNE") {
        app::prune(path)
    } else if args.is_present("TASK") {
        let description = get_description(args)?;
        app::add(path, &description)
    } else {
        app::print(path)
    }
}

fn parse_sequence(s: &str) -> Result<usize> {
    let sequence = s.parse::<usize>()?;
    if sequence > 0 {
        Ok(sequence - 1)
    } else {
        Err(Error::InvalidSequence)
    }
}

fn get_description(args: &ArgMatches) -> Result<String> {
    let description = args.value_of("TASK");
    match description {
        None => Err(Error::MissingDescription),
        Some(desc) => Ok(desc.into()),
    }
}
