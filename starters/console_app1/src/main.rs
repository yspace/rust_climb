use wena::*;

fn main() {
    Application::new("calculator")
        .commands([Command::new("sum")
            .description("Add two numbers")
            .definition([
                Argument::new("first").required(true),
                Argument::new("second").required(true),
            ])
            .handler(|app| {
                let first = app.input.argument::<i32>("first").unwrap();
                let second = app.input.argument::<i32>("second").unwrap();

                app.output.writeln(
                    Alert::info(
                        format!("Total: {}", first + second)
                    )
                );

                Ok(0)
            })])
        .run();
}