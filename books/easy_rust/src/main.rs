use wena::*;

mod routes;
mod chapters ;

fn main() {
    Application::new("easy rust")
        .commands([commands::cmd_route(), commands::add_cmd()])
        .run();
}

mod commands {
    use whale::route::router::Router;

    use super::*;

    pub fn cmd_route() -> Command {
        Command::new("r")
            .description("route to chapter")
            .definition([Argument::new("chapter_path").required(false)])
            .handler(|app| {
                let chapter_path = app.input.argument::<String>("chapter_path");

                //println!("route is {}", chapter_path);
                

                let mut router = Router::new();
                routes::configure(&mut router);

                if let Some(chapter_path) = chapter_path {
                    println!("route to {}", chapter_path);
                    let act_key = chapter_path.as_str();
                    router.handle(&act_key);

                   return Ok(0);
                }

                // println!("{:#?}", router);
                //println!("{:#?}", router.descendents(""));

                Ok(0)
            })
    }

    pub fn add_cmd() -> Command {
        Command::new("sum")
            .description("Add two numbers")
            .definition([
                Argument::new("first").required(true),
                Argument::new("second").required(true),
            ])
            .handler(|app| {
                let first = app.input.argument::<i32>("first").unwrap();
                let second = app.input.argument::<i32>("second").unwrap();

                app.output
                    .writeln(Alert::info(format!("Total: {}", first + second)));

                Ok(0)
            })
    }
}
