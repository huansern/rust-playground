use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook::iterator::Signals;
use std::io::Error;

fn main() -> Result<(), Error> {
    let signals = vec![SIGTERM, SIGQUIT, SIGINT, SIGHUP];
    let mut signal = Signals::new(&signals)?;
    for s in signal.forever() {
        match s {
            SIGTERM => {
                println!("Received SIGTERM. Terminating...");
                break;
            }
            SIGQUIT => {
                println!("Received SIGQUIT. Terminating...");
                break;
            }
            SIGINT => {
                println!("Received SIGINT. Terminating...");
                break;
            }
            SIGHUP => {
                println!("Received SIGHUP. Reloading configuration from disk...");
            }
            i => println!("Received unhandled signal {}.", i),
        }
    }
    Ok(())
}
