use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod router;

mod iterators;

mod data_types ;
mod basic_data_types ;

mod lifetimes ;

mod anys;
mod arrays;

// 二进制
mod bits ;
// 异步
mod asyncs;
mod tokios ;

mod closures;
mod collections;
mod copy_move;
// Continuation-passing style 
mod cps ;
mod drops;
mod ecs;
mod envs;
mod errors;
mod files;
mod fmts;
mod funcs;
mod generics;
mod hashmap;
mod impl_traits;
mod impls;
mod macros;
mod pattern_matches;
mod sized;
mod slices;
mod strings;
mod structs;
mod threads;
mod vectors;
// 常量
mod consts;
mod statics;
// 流程控制
mod control_flows;

// 日期 时间
mod date_times;

// 树
mod trees;

mod serdes ;

mod combinators;

// 配置
mod configs ;

//
mod boxes ;

mod enums;
mod arcs;

fn init() {
    println!("init fn of crate hardway");
}

mod routes {
    use crate::router::Router;
    pub fn configure(router: &mut Router) {
        router.insert("/", || {
            println!("fn: /");
        });
    }
}

// dead_code 死代码 即未被使用到的孤立代码元素
#[allow(dead_code,unused_variables)]
fn main() {
    std::env::set_var("ENV_TEMP_VAR", "env_value");

    // cmd: cargo run -p hardway --  --act=<some_action>
    _seahorse_main();
}

fn _seahorse_main() {
    use seahorse::{App, Command, Context, Flag, FlagType};
    use std::env;

    let args: Vec<String> = env::args().collect();

    // let some_integer = 0 ;

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .flag(Flag::new("act", FlagType::String).description("which action to be executedß"))
        .flag(
            Flag::new("help", FlagType::String)
                .description("show the usage information for the command"),
        )
        .action(|c| {
            println!(" > Hello, {:?} this is the default action . \n\n", c.args);

            match c.string_flag("act") {
                Ok(act) => {
                    println!("resolved action is {}", act);
                    // callbacks.call(&act) ;
                    routers::run(&act);
                }
                _ => {
                    // callbacks.call(&"strings");
                    if c.args.len() > 0 {
                        let mut module_entries = CallbacksMut::new();
                        module_entries.register("vectors".to_string(), vectors::main);
                        module_entries.register("consts".to_string(), consts::main);
                        module_entries.register("statics".to_string(), statics::main);
                        module_entries.register("control_flows".to_string(), control_flows::main);
                        module_entries.register("trees".to_string(), trees::main);
                        module_entries.register("date_times".to_string(), date_times::run );
                        module_entries.register("cps".to_string(), cps::main );
                        module_entries.register("tokios".to_string(), tokios::main );
                        module_entries.register("files".to_string(), files::run );
                        module_entries.register("boxes".to_string(), boxes::main  );
                        module_entries.register("basic_data_types".to_string(), basic_data_types::main    );
                        module_entries.register("anys".to_string(), anys::main    );
                        module_entries.register("lifetimes".to_string(), lifetimes::main    );
                        module_entries.register("arcs".to_string(), arcs::run    );

                        let act_key = c.args[0].as_str();
                        if module_entries.is_key_exists(act_key) {
                            println!("\r\n==== call {} < ====\r\n",act_key);
                            module_entries.call(act_key);
                            println!("\r\n==== end the {} > ====",act_key);
                        }else{
                            println!(">> *** not found: {} \r\n",act_key);
                        }
                    } else {
                        println!("some error happened");
                    }
                }
            }

            println!("\n\n > / end defalt action, args: {:?}", c.args);
        })
        .command(
            Command::new("help")
                .description("need help?")
                .alias("h")
                .usage("cli help(h) [...]"),
        )
        .command(
            Command::new("itr")
                .description("iterators?")
                .usage("cargo run -p hardway itr")
                .action(|_c: &Context| {
                    println!("------------iterators -------------");
                    iterators::main();
                    // println!("capture some external variables {}", some_integer) ;
                }),
        )
        .command(
            Command::new("std-io")
                .usage("cargo run -p hardway std-io")
                .description("rust的输入和格式化输出")
                .action(|c: &Context| {
                    use std::io;
                    println!("请输入姓名:");
                    let mut name = String::new();
                    //读取一个字符串
                    io::stdin().read_line(&mut name);
                    //必须使用占位符
                    println!("你好! {}", name);
                }),
        )
        .command(
            Command::new("threads")
                .usage("cargo run -p hardway threads")
                .description("rust 中线程的基本使用")
                .action(|c: &Context| {
                    threads::main();
                }),
        )
        .command(
            Command::new("strings")
                .usage("cargo run -p hardway strings") //macos: cargo run -p hardway -- strings
                .description("rust 中字符串的基本使用")
                .action(|_c: &Context| {
                    strings::main();
                }),
        )
        .command(
            Command::new("sized")
                .usage("cargo run -p hardway sized") //macos: cargo run -p hardway -- sized
                .description("rust sized trait")
                .action(|_c: &Context| {
                    sized::main();
                }),
        )
        .command(
            Command::new("slices")
                .usage("cargo run -p hardway slices") //macos: cargo run -p hardway -- slices
                .description("rust 切片")
                .action(|_c: &Context| {
                    slices::main();
                }),
        )
        .command(
            Command::new("hashmap")
                .usage("cargo run -p hardway hashmap") //macos: cargo run -p hardway -- slices
                .description("rust hashmap")
                .action(|_c: &Context| {
                    hashmap::main();
                }),
        )
        .command(
            Command::new("funcs")
                .usage("cargo run -p hardway -- funcs") //macos: cargo run -p hardway -- slices
                .description("rust funcs")
                .action(|_c: &Context| {
                    funcs::main();
                }),
        )
        .command(
            Command::new("closures")
                .usage("cargo run -p hardway -- closures") //macos: cargo run -p hardway -- slices
                .description("rust closures")
                .action(|_c: &Context| {
                    closures::main();
                }),
        )
        .command(
            Command::new("collections")
                .usage("cargo run -p hardway -- collections") //macos: cargo run -p hardway -- slices
                .description("rust collections")
                .action(|_c: &Context| {
                    collections::main();
                }),
        )
        .command(
            Command::new("pattern_matches")
                .usage("cargo run -p hardway -- pattern_matches") //macos: cargo run -p hardway -- slices
                .description("rust pattern_matches")
                .action(|_c: &Context| {
                    pattern_matches::main();
                }),
        )
        .command(
            Command::new("ecs")
                .usage("cargo run -p hardway -- ecs") //macos: cargo run -p hardway -- slices
                .description("rust ecs leaning")
                .action(|_c: &Context| {
                    ecs::main();
                }),
        )
        .command(
            Command::new("errors")
                .usage("cargo run -p hardway -- errors") //macos: cargo run -p hardway -- slices
                .description("rust error handling")
                .action(|_c: &Context| {
                    errors::main();
                }),
        )
        .command(
            Command::new("asyncs")
                .usage("cargo run -p hardway -- asyncs") //macos: cargo run -p hardway -- slices
                .description("rust async ")
                .action(|_c: &Context| {
                    // https://course.rs/async/intro.html
                    asyncs::main();
                }),
        )
        .command(
            Command::new("generics")
                .usage("cargo run -p hardway -- generics") //macos: cargo run -p hardway -- slices
                .description("rust generics ")
                .action(|_c: &Context| {
                    // https://course.rs/async/intro.html
                    generics::main();
                }),
        )
        .command(
            Command::new("copy")
                .usage("cargo run -p hardway -- copy") //macos: cargo run -p hardway -- slices
                .description("rust copy move ")
                .action(|_c: &Context| {
                    copy_move::main();
                }),
        )
        .command(
            Command::new("macros")
                .usage("cargo run -p hardway -- macros") //macos: cargo run -p hardway -- slices
                .description("rust macros ")
                .action(|_c: &Context| {
                    // https://course.rs/async/intro.html
                    macros::main();
                    // macros::pub_macros::my_macros!() ;
                    crate::my_macros!(); // 子模块中的宏定义被导入到了根crate下了

                    fn init() {
                        println!("my init function");
                    }
                    // 仔细区别下面两种宏
                    crate::do_func_init!();
                    crate::do_crate_func_init!();

                    // 内用规则
                    crate::m_action!();
                    crate::m_action2!();
                }),
        )
        .command(
            Command::new("fmt")
                .usage("cargo run -p hardway -- fmt") //macos: cargo run -p hardway -- slices
                .description("rust fmt ")
                .action(|_c: &Context| {
                    fmts::main();
                }),
        )
        .command(
            Command::new("env")
                .usage("cargo run -p hardway -- env") //macos: cargo run -p hardway -- slices
                .description("rust env ")
                .action(|_c: &Context| {
                    envs::run();
                }),
        )
        .command(
            Command::new("structs")
                .usage("cargo run -p hardway -- structs") //macos: cargo run -p hardway -- slices
                .description("structs ")
                .action(|_c: &Context| {
                    structs::run();
                }),
        )
        .command(
            Command::new("arrays")
                .usage("cargo run -p hardway -- arrays") //macos: cargo run -p hardway -- slices
                .description("arrays in rust ")
                .action(|_c: &Context| {
                    arrays::run();
                }),
        )
        .command(
            Command::new("impls")
                .usage("cargo run -p hardway -- impls") //macos: cargo run -p hardway -- slices
                .description("impl keyword in rust ")
                .action(|_c: &Context| {
                    impls::run();
                }),
        );

    app.run(args);
}
// =================================================================
mod routers {
    use super::*;

    pub fn run(act: &str) {
        let mut callbacks = CallbacksMut::new();
        // cargo run -p hardway -- --act=strings
        callbacks.register("strings".to_string(), || strings::main());

        callbacks.call(act);
    }
}

struct Callbacks {
    callbacks: HashMap<String, Box<dyn Fn()>>,
}

impl Callbacks {
    pub fn new() -> Self {
        Callbacks {
            callbacks: HashMap::new(),
        }
    }

    pub fn register(&mut self, k: &str, callback: Box<dyn Fn()>) {
        self.callbacks.insert(k.to_string(), callback);
    }

    pub fn call(&mut self, k: &str) {
        let cb = self.callbacks.get(k);
        match cb {
            Some(cb) => cb(),
            None => println!("can't find the callback:<{}> in the callback list", k),
        }
    }
}
#[derive(Clone)]
struct CallbacksMut {
    callbacks: HashMap<String, Rc<RefCell<dyn FnMut()>>>,
    // callbacks: HashMap<String,  Rc<RefCell<FnMut()>>>,
}

impl CallbacksMut {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }

    pub fn register0<F: FnMut() + 'static>(&mut self, k: String, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.insert(k, cell);
    }
    pub fn register<F: FnMut() + 'static>(&mut self, k: String, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.insert(k, cell);
    }

    pub fn is_key_exists(&mut self, k: &str) -> bool {
        self.callbacks.contains_key(k)
    }

    pub fn call(&mut self, k: &str) {
        let cb = self.callbacks.get(k);
        match cb {
            Some(cb) => {
                let mut mut_closure = cb.borrow_mut();
                (&mut *mut_closure)();
            }
            None => println!("can't find the callback:<{}> in the callback list", k),
        }
    }
}

// 测试下Fn FnMut

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fn_types() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_callbacks_mut() {
        let mut c = CallbacksMut::new();
        c.register("foo".to_string(), || println!("Callback 1: foo"));
        c.call(&"foo");

        {
            let mut count: usize = 0;
            c.register("bar".to_string(), move || {
                count = count + 1;
                println!("Callback 2:  ({}. time)", count);
            });
        }
        c.call(&"foo");
        c.clone().call(&"bar");
        c.call(&"foo");
        c.clone().call(&"bar");
    }
    #[test]
    fn test_is_key_exists() {
        let mut c = CallbacksMut::new();
        c.register("foo".to_string(), || println!("Callback 1: foo"));
        assert_eq!(c.is_key_exists(&"foo"), true);
    }
}
