pub mod my_module;
pub mod command_loader;

use clap::Command;

pub trait Module {
       fn get_commands(&self) -> Vec<Command>;
       
}

pub struct App{
    // components: Vec<Box<dyn Component>>,
    modules: Vec<Box<dyn Module>>, 
}
impl App{
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
          }
    }

    pub fn add(&mut self, module: impl Module + 'static) {
        self.modules.push(Box::new(module));
    }
}

impl Module for App {
        fn get_commands(&self) -> Vec<Command> {
        let mut cmd =  cli() ;
        for module in &self.modules {
           cmd = cmd.subcommands(module.get_commands());
        }
        vec![
           cmd,
        ]
        
    }
}

fn cli() -> Command {
   
   

    Command::new("hi_clap")
        
        
        .subcommand(
            Command::new("ping")
                .about("Get a response")
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
        )
}