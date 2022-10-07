pub fn main() {
    println!("hi this is demo for handling error in rust");
    // https://www.youtube.com/watch?v=UgIQo__luHw&ab_channel=BoyMaas
}

mod anyhow_demo {

    #[derive(Debug)]
    pub enum MyError {
        IoError(String),
        Inexist(String),
    }

    pub type Result<T> = std::result::Result<T, MyError>;

    // pub fn fetch_id() -> Result<u64> {
    //     let content = std::fs::read_to_string("/tmp/tmp_id")?;
    //     let id = content.parse::<u64>()?;
    //     Ok(id)
    // }
}

mod thiserror_demo {}

mod error_chain_demo {}

mod color_eyre_demo{
    fn run() -> color_eyre::Result<()> {
 // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    
    Ok(())
    }
}
