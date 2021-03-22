use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook::iterator::Signals;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Set terminate to true and return upon receiving SIGTERM, SIGQUIT or SIGINT signals.
fn signal_handler(terminate: Arc<AtomicBool>) -> Result<(), Error> {
    let signals = vec![SIGTERM, SIGQUIT, SIGINT, SIGHUP];
    let mut signal = Signals::new(&signals)?;
    for s in signal.forever() {
        match s {
            SIGTERM => {
                println!("Received SIGTERM. Terminating...");
                terminate.store(true, Ordering::Relaxed);
                break;
            }
            SIGQUIT => {
                println!("Received SIGQUIT. Terminating...");
                terminate.store(true, Ordering::Relaxed);
                break;
            }
            SIGINT => {
                println!("Received SIGINT. Terminating...");
                terminate.store(true, Ordering::Relaxed);
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

// Worker function returns when should_terminate set to ture
fn worker(id: i32, should_terminate: Arc<AtomicBool>) {
    for i in 0..10 {
        if should_terminate.load(Ordering::Relaxed) {
            println!("Worker {} exited.", id);
            break;
        }
        thread::sleep(Duration::from_millis(1000));
        println!("Worker #{}: Completed task #{}", id, i + 1);
    }
}

fn main() {
    // Shared state across threads, signaling worker threads to return when it set to true
    let terminate = Arc::new(AtomicBool::new(false));
    let should_terminate = terminate.clone();
    // Spawn a thread to intercept incoming signal, set terminate to true on SIGTERM
    thread::spawn(move || signal_handler(should_terminate));

    // Spawn a number of worker threads to simulate some task, workers return when terminate set to true
    let mut handles = vec![];
    for i in 0..10 {
        let should_terminate = terminate.clone();
        handles.push(thread::spawn(move || worker(i + 1, should_terminate)));
    }

    // Wait for all worker threads to return before returning main
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All worker threads returned, returning main.");
}
