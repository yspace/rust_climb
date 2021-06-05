use crossterm::cursor::MoveRight;
use std::io::Stdout;
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


fn print_message(stdout: &mut Stdout, msg: &str) -> Result<()> {
    stdout.queue(ScrollUp(1))?
    .queue(MoveToColumn(1))?
    // .queue(Print("Our buffer: "))?
    .queue(Print(msg))?
    .queue(ScrollUp(1))?
    .queue(MoveToColumn(1))? ;
    stdout.flush()?;
    
    Ok(())
}

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
                                
                                print_message(& mut stdout,
                                    &format!("Our buffoer {}",buffer)
                                )? ;
                                
                                buffer.clear() ;
                                break 'input;
                            }
                        }
                        KeyCode::Left => {
                            // print_message(&mut stdout,"Left!")? ;
                            stdout.queue(
                                MoveLeft(1)
                            )?; 
                            stdout.flush()? ;
                        }
                        KeyCode::Right => {
                            // print_message(&mut stdout,"Right!")? ;
                            stdout.queue(
                                MoveRight(1)
                            )?; 
                            stdout.flush()? ;
                        }
                        _ => {}
                    }
                }
                Event::Mouse(event) => {
                    print_message(&mut stdout, &format!("{:?}", event))?;
                }
                Event::Resize(width, height) => {
                    print_message(&mut stdout , &format!("width: {}, height:{}", width, height))? ;
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
