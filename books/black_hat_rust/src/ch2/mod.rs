use thiserror::Error;
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <target.com>")]
    CliUsage,
}


pub fn main() -> Result<(), anyhow::Error> {
    Ok(())
}