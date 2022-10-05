use wena::*;

use super::* ;
use whale::route::router::Router;

pub fn to(/* some_dep: SomeType */) -> Command {

    let command = Command::new("to")
    .definition([
        Argument::new("route").required(true),
    ]).handler(|app| {
        let value = app.input.argument::<String>("route");

        let mut router = Router::new();
            routes::configure(&mut router);

            sections::register_routes(&mut router);

            if value.is_some() {
                let act_key = value.unwrap();
                let rslt = router.handle(&act_key);
                match rslt {
                    Ok(()) => {},
                    Err(err) => println!("{}",err) ,
                }

               
            }

           // println!("Hello, {:?}", c.args);
            // router.handle("/");
            
            // println!("{:#?}", router);
            use colored::Colorize;
// 官方例子有bug了  参数类型有问题了
// app.output.writeln("My message".bold().italic().green());   
            println!(" {}: \n\n {:#?}","all routes".bold().italic().green(), router.descendents(""));



        Ok(0)
    });

    command
}