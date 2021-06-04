use crossterm::cursor::MoveLeft;
use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;

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
        // .execute(SetBackgroundColor(Color::Red))?
        // .execute(Print("Styled text here."))?
        .execute(Print(">"))?
        .execute(ResetColor)? ;
        // .execute(EnableMouseCapture)?;
    
        terminal::enable_raw_mode()?; 
       
        let mut buffer = String::new() ;

        loop {
            match read()? {
            Event::Key(KeyEvent{code, modifiers}) => {
                // println!("{:?} , {:?}", code,modifiers) ;
                match code {
                    KeyCode::Char(c) => {
                        //stdout().write_all(buffer.as_bytes())? ;
                        let mut char_buffer = [0; 4] ;
                        let bytes = c.encode_utf8(&mut char_buffer).as_bytes() ;
                        stdout().write_all(&bytes)? ;
                        stdout().flush()? ;
                        
                        buffer.push(c) ;
                    }
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            buffer.pop() ;
                            stdout().execute(MoveLeft(1))? ;
                            // stdout().write_all(&[b' '] )? ;
                            stdout().write_all( b" " )? ;
                            stdout().execute(MoveLeft(1))? ;
                            stdout().flush()? ;
                        }
                    }
                    KeyCode::Enter => {
                        break ;
                    }
                    _ => {}
                }
            }
            Event::Mouse(event) => {
                println!("{:?}", event) ;
            }
            Event::Resize(width, height) => {
                println!("width: {}, height:{}", width,height) ;
            }
        }
 
        }

       println!("our buffer: {}", buffer) ;
        // stdout().execute(DisableMouseCapture)?;
       terminal::disable_raw_mode()? ;
    Ok(())
}
