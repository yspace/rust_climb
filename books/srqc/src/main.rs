mod ch1 ;
mod ch2 ;
mod ch3 ;
mod ch4 ;
mod ch5 ;

fn main() {
//   ch1::hello::main() ;
//    ch2::vars::main() ;
//    ch2::basic_types::main() ;
    // cargo run -p srqc -- --help
    _seahorse_main();

//    _clap_main() ;
}

// ==================================================================
//      ## copy from https://github.com/guoxbin/dtool/blob/master/src/main.rs
mod app;
mod modules;
fn _clap_main(){

    let (app, module_manager) = app::build_app();

    let mut app_clone = app.clone();

    let matches = app.get_matches();

    let (name, matches) = matches.subcommand();

    if let Some(matches) = matches {
        module_manager.run(name, matches);
    } else {
        app_clone.print_help().unwrap_or(());
        println!();
    }
}

fn _seahorse_main(){
    use seahorse::{App,Command,Context,Flag,FlagType};
    use std::env;

    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        .command( Command::new("help")
            .description("need help?")
            .alias("h")
            .usage("cli help(h) [...]")
            .action(|c: &Context| println!("{:?}", c.args)))
        .command(
            // NOTE 这里如果想同时适配seahorse 跟clap两个不同的cli库 中间需要某种适配层 参考六边形架构中的port/adapter
            Command::new("some-chapter")
                .usage("cargo run -p srqc ch2 --route 2")
            .description("深入浅出第二章")
            .action(|c: &Context|{
                match c.string_flag("route") {
                    Ok(to) => {
//                        let sum: i32 = match &*op {
//                            "add" => c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum(),
//                            "sub" => c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum(),
//                            _ => panic!("undefined operator..."),
//                        };
//
//                        println!("{}", sum);
                         match &*to{
                           "1" => println!("第一节"),
                           "2" => println!("第二节"),
                            _ => println!("默认节"),

                        };
                    }
                    Err(e) => {
                         panic!(format!("error happens : {}", e));
                    }
//                    Err(e) => match e {
//                        FlagError::Undefined => panic!("undefined operator..."),
//                        FlagError::ArgumentError => panic!("argument error..."),
//                        FlagError::NotFound => panic!("not found flag..."),
//                        FlagError::ValueTypeError => panic!("value type mismatch..."),
//                        FlagError::TypeError => panic!("flag type mismatch..."),

//                    },
                }

                // ch2::basic_types::main() ;
            })
                .flag(
                    Flag::new("route", FlagType::String)
                          .description("route flag")
                          .alias("r")
                )
        ).command(
            // NOTE 这里如果想同时适配seahorse 跟clap两个不同的cli库 中间需要某种适配层 参考六边形架构中的port/adapter
            Command::new("ch2")
                .usage("cargo run -p srqc ch2 --route 2")
            .description("深入浅出第二章")
            .action(|c: &Context|{
                 ch2::basic_types::main() ;
                 ch2::compound_types::main() ;
            })

        )
        .command(
            Command::new("ch3")
                .usage("cargo run -p srqc ch3")
            .description("深入浅出第三章")
            .action(|c: &Context|{
                ch3::expression::main() ;
                ch3::if_else::main() ;
                ch3::while_stmt::main() ;
                ch3::for_loop::main() ;
            })
        )
        .command(
            Command::new("ch4")
                .usage("cargo run -p srqc ch4")
            .description("深入浅出第四章")
            .action(|c: &Context|{
                ch4::func::main() ;
            })
        )
        .command(
            Command::new("ch5")
                .usage("cargo run -p srqc ch5")
            .description("深入浅出第五章")
            .action(|c: &Context|{
                ch5::traits::main() ;
            })
        )
        ;

    app.run(args);
}

use std::collections::HashMap;
#[derive(Default)]
struct Runner {
    // 参考：https://github.com/redox-os/orbtk/.../crates/api/src/widget_base/registry.rs
    funcs: HashMap<  String ,  Box<dyn Fn()> >
}

impl Runner{
    /// Creates a service registry with an empty Registry map.
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(){

    }
    pub fn run(){

    }

    pub fn add_case(&mut self, key: impl Into<String>, func:   Box<dyn Fn()> ){
        self.funcs.insert(key.into(), func) ;
    }
    pub fn get_case(&self, key: &str) -> Option< &Box<dyn Fn() > > {
        let func = self.funcs
            .get(&key.to_string()) ;
//            .unwrap_or_else(|| panic!("Registry.get(): key: {} could not be found.", key))

        match func {
            Some(f) =>  Some(f),
            None => None ,
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn runner_get() {
        let mut runner = Runner::new();
        fn one(){
            println!("one called") ;
        }
        runner.add_case("one", Box::new(one)) ;

        assert!(runner.get_case ("one").is_some());
        //  runner.get_case("one").unwrap()() ;
    }




}