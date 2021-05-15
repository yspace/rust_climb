use crate::modules::{base, Command, Module};
use clap::{Arg, ArgMatches, SubCommand};


pub fn module<'a, 'b>() -> Module<'a, 'b> {
    Module {
        desc: "chapter2 of 深入浅出".to_string(),
        commands: commands(),
        get_cases: cases::cases,
    }
}

pub fn commands<'a, 'b>() -> Vec<Command<'a, 'b>> {
    vec![
        Command {
            app: SubCommand::with_name("ch2")
                .about("chapter2 of 深入浅出"),
              //  .arg(Arg::with_name("INPUT").required(false).index(1)),
            f: action,
        },

    ]
}

fn action(matches: &ArgMatches)
     -> Result<Vec<String>, String>
   // -> Result<(), String>
{
//    let input = base::input_string(matches)?; // NOTE 这里是从控制台输入参数哦

    println!("hillo") ;
    Ok(Vec::new())
}

mod cases {

    use crate::modules::Case;
    use linked_hash_map::LinkedHashMap;
//    use std::iter::empty;

    pub fn cases() -> LinkedHashMap<&'static str, Vec<Case>> {
        LinkedHashMap::new()
    }
}
