use anyhow::Result ;

pub fn main() {
    let result = run() ;
    // ...actual work...
    match result {
        Ok(_) => {
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        // Err(CustomError::CantReadConfig(e)) => {
        //     eprintln!("Error: {}", e);
        //     std::process::exit(exitcode::CONFIG);
        // }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}

fn run() -> Result<()> {
    Ok(())
}