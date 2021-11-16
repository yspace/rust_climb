use crossterm::cursor::MoveLeft;
use crossterm::cursor::MoveRight;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::MoveToNextLine;
use crossterm::cursor ;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::terminal::ScrollUp;
use crossterm::QueueableCommand;
use std::io::Stdout;

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
    stdout
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?
        // .queue(Print("Our buffer: "))?
        .queue(Print(msg))?
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?;
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
    let mut caret_pos: u16 ;

    'repl: loop {
        // print the prompt
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(">"))?
            .execute(ResetColor)?;
        // set where the input begins . 
        // let (input_start_col, input_start_row) = cursor::position()? ;
        let (input_start_col, _) = cursor::position()? ;
        caret_pos = input_start_col ;

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
                            caret_pos +=1 ;
                        }
                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(" "))?
                                    .queue(MoveLeft(1))?;

                                stdout.flush()?;

                                caret_pos -= 1 ;
                            }
                        }
                        KeyCode::Enter => {
                            if buffer == "exit" {
                                break 'repl;
                            } else {
                                // println!("Our buffer: {}", buffer) ;

                                // stdout.queue(MoveToNextLine(1))?

                                print_message(&mut stdout, &format!("Our buffoer {}", buffer))?;

                                buffer.clear();
                                break 'input;
                            }
                        }
                        KeyCode::Left => {
                            // print_message(&mut stdout,"Left!")? ;
                            if caret_pos > input_start_col {
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;

                                caret_pos -= 1 ;
                            }
                            
                        }
                        KeyCode::Right => {
                            // print_message(&mut stdout,"Right!")? ;
                            if (caret_pos as usize) < ((input_start_col as usize) + buffer.len()) {
                                stdout.queue(MoveRight(1))?;
                                stdout.flush()?;
                                
                                caret_pos += 1 ;
                            }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(event) => {
                    print_message(&mut stdout, &format!("{:?}", event))?;
                }
                Event::Resize(width, height) => {
                    print_message(&mut stdout, &format!("width: {}, height:{}", width, height))?;
                }
            }
        }
    }

    // println!("our buffer: {}", buffer);
    // stdout().execute(DisableMouseCapture)?;
    terminal::disable_raw_mode()?;

    println!();
    Ok(())
}
