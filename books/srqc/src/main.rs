//#[allow(dead_code)]
//或者你添加一个条箱级别（在你的主箱子里），注意! ：
#![allow(dead_code)]
#![allow(unused_variables)]

mod ch1 ;
mod ch2 ;
mod ch3 ;
mod ch4 ;
mod ch5 ;
mod ch6 ;
mod ch7 ;
mod ch8 ;
mod ch9 ;
mod ch11 ;
mod ch12 ;
mod ch13 ;
mod ch14 ;
mod ch15 ;
mod ch16 ;

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
        .command(
            Command::new("ch6")
                .usage("cargo run -p srqc ch6")
            .description("深入浅出第六章")
            .action(|c: &Context|{
//                ch6::arrays::main() ;
                ch6::strings::main() ;
            })
        )
        .command(
            Command::new("ch7")
                .usage("cargo run -p srqc ch7")
            .description("深入浅出第七章")
            .action(|c: &Context|{
                ch7::pattern_destructure::main() ;
            })
        )
        // .command(
        //     Command::new("ch8")
        //         .usage("cargo run -p srqc ch8")
        //     .description("深入浅出第八章")
        //     .action(|c: &Context|{
        //         ch8::main() ;
        //     })
        // )
       
        .command(
            Command::new("ch9")
                .usage("cargo run -p srqc ch9")
            .description("深入浅出第九章")
            .action(|c: &Context|{
                ch9::main() ;
            })
        )
        .command(
            Command::new("ch11")
                .usage("cargo run -p srqc ch11")
            .description("深入浅出第11章：所有权和移动")
            .action(|c: &Context|{
                ch11::main() ;
            })
        )
        .command(
            Command::new("ch12")
                .usage("cargo run -p srqc ch12")
            .description("深入浅出第12章：所有权和移动")
            .action(|c: &Context|{
                ch12::main() ;
            })
        )
        .command(
            Command::new("ch13")
                .usage("cargo run -p srqc ch13")
            .description("深入浅出第13章：借用检查")
            .action(|c: &Context|{
                ch13::main() ;
            })
        )
        .command(
            Command::new("ch14")
                .usage("cargo run -p srqc ch14")
            .description("深入浅出第14章：NLL")
            .action(|c: &Context|{
                ch14::main() ;
            })
        )
        .command(
            Command::new("ch15")
                .usage("cargo run -p srqc ch15")
            .description("深入浅出第15章：内部可变性")
            .action(|c: &Context|{
                ch15::main() ;
            })
        )
        .command(
            Command::new("ch16")
                .usage("cargo run -p srqc ch16")
            .description("深入浅出第16章：解引用")
            .action(|_c: &Context|{
                ch16::main() ;
            })
        )
        ;

    app.run(args);
}


pub mod cli {
    use whale::util::runner::* ;
    use std::env ;

    pub  fn get_route(name: String) -> Option<String> {
//        let args: Vec<String>  = env::args().collect() ;
//
//        let mut idx = 0 ;
//        for arg in &args {
//            if name == arg {
//                return Some(args[idx+1]. clone()) ;
//            }
//
//            idx +=1 ;
//        }

        None
    }

    pub trait CliRunner{

        fn run_route(&mut self , r: String);
    }

    impl CliRunner for Runner{
        fn run_route(&mut self , r: String) {
            println!("run route: {}", r) ;
        }
    }

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