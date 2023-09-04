use wena::{Alert, Output};
use wena::{Argument, Command, Input};

use crate::CONFIG;
use crate::settings::Settings;

pub fn build_command(config: Settings) -> Command {
    let command = Command::new("greet")
        .definition([Argument::new("to").required(true)])
        .description("打个招呼 😂")
        .handler(|app| {
            let value = app.input.argument::<String>("to");

            println!("hello {} 😄", value.unwrap());
            //  println!("config: {:?}",config);
            println!("config: {:?}",CONFIG.log);

            app.output
                .writeln(Alert::error("This is a error — check it out!"));
            app.output
                .writeln(Alert::info("This is a info — check it out!"));
            app.output
                .writeln(Alert::warning("This is a warning — check it out!"));

            Ok(0)
        });

    return command;
}
