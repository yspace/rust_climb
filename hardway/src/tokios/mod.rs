use tokio;

pub fn main()   {
   _main() ;
}

fn _main() -> color_eyre::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    rt.block_on(run())
}

async fn run() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    println!("hi from tokios") ;

    Ok(())

}