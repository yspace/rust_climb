use super::*;

pub struct MyModule{}
impl MyModule{
    pub fn new() -> Self {
        Self {  }
    }
}

impl Module for MyModule {
        fn get_commands(&self) -> Vec<Command> {

        vec![
            cli_0(),
        ]
        
    }
}

fn cli_0() -> Command {
    Command::new("user")
    .about("user manager")
}