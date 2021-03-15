use std::{env, fs, io};

mod app;
mod view;

fn main() -> io::Result<()> {
    let path = match env::args().nth(1) {
        None => env::current_dir()?,
        Some(input) => fs::canonicalize(input)?,
    };
    let mut app = app::App::new(path)?;
    let mut siv = cursive::default();
    view::initialise(&mut siv);
    view::default_theme(&mut siv);
    match view::refresh(&mut siv, &mut app, "") {
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
        _ => {}
    };
    siv.set_user_data(app);
    siv.run();
    Ok(())
}
