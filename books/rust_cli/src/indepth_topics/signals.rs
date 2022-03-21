
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};

pub fn run() {
    let result = match main() {
        Ok(())=>{ println!("exit main!") }, 
        Err(err)=> { println!("{err}")},
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
         
            println!("Received signal {:?}", sig);
        }
    });

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    // thread::sleep(Duration::from_secs(2));
    thread::sleep(Duration::from_secs(10));

    Ok(())
}