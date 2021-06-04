use crossterm::cursor::MoveToColumn;
use crossterm::terminal::ScrollUp;
use crossterm::cursor::MoveToNextLine;
use crossterm::cursor::MoveLeft;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::QueueableCommand;

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use std::io::{stdout, Write};

use crossterm::{
    event::{read, Event},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
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
    let mut stdout = stdout();
    

    terminal::enable_raw_mode()?;

    let mut buffer = String::new();

   'repl: loop {
        // print the prompt
        stdout
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print(">"))?
        .execute(ResetColor)?;

       'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    // println!("{:?} , {:?}", code,modifiers) ;
                    match code {
                        KeyCode::Char(c) => {
                            //stdout().write_all(buffer.as_bytes())? ;
                            // let mut char_buffer = [0; 4] ;
                            // let bytes = c.encode_utf8(&mut char_buffer).as_bytes() ;
                            // stdout.write_all(&bytes)? ;
                            stdout.queue(Print(c))?;
                            stdout.flush()?;

                            buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(" "))?
                                    .queue(MoveLeft(1))?;

                                stdout.flush()?;
                            }
                        }
                        KeyCode::Enter => {
                            if buffer == "exit" {
                                break 'repl ;
                            }else{
                                // println!("Our buffer: {}", buffer) ;
                                
                                // stdout.queue(MoveToNextLine(1))?
                                stdout.queue(ScrollUp(1))?
                                .queue(MoveToColumn(1))?
                                .queue(Print("Our buffer: "))?
                                .queue(Print(&buffer))?
                                .queue(ScrollUp(1))?
                                .queue(MoveToColumn(1))? ;
                                stdout.flush()?;
                                
                                buffer.clear() ;
                                break 'input;
                            }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(event) => {
                    println!("{:?}", event);
                }
                Event::Resize(width, height) => {
                    println!("width: {}, height:{}", width, height);
                }
            }
        }
    }

    // println!("our buffer: {}", buffer);
    // stdout().execute(DisableMouseCapture)?;
    terminal::disable_raw_mode()?;

    println!() ;
    Ok(())
}
