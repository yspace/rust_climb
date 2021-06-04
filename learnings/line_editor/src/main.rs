use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event::{read,Event},
    terminal,
};

fn main() -> Result<()> {
    // using the macro
    // execute!(
    //     stdout(),
    //     SetForegroundColor(Color::Blue),
    //     SetBackgroundColor(Color::Red),
    //     Print("Styled text here."),
    //     ResetColor
    // )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?
        .execute(EnableMouseCapture)?;
    
        terminal::enable_raw_mode()?; 
       
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event) ;
            }
            Event::Mouse(event) => {
                println!("{:?}", event) ;
            }
            Event::Resize(width, height) => {
                println!("width: {}, height:{}", width,height) ;
            }
        }

        stdout().execute(DisableMouseCapture)?;

    Ok(())
}
