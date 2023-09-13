use seahorse::{Command, Context};

use crate::CONFIG;
use crate::settings::Settings;

pub fn build_command(config: Settings) -> Command {
    
    Command::new("run")
    .description("Run a spider")
    .alias("r")
    .usage("cli run")
    .action(|c: &Context|{
        println!("Hello from greet cmd, {:?}", c.args);
        if let Some(app_data) =  c.extensions_mut().get::<Settings>(){

            println!("app-data: {:#?}", app_data) ;
        }
    })

}
