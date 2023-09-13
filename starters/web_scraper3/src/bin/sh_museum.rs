use seahorse::{App, Context, Command};
use std::env;
use dotenv::dotenv ;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .command(add_command())
        .command(env_command())
        .command(config_command())
        .command(sub_command());

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}

fn add_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
}

fn add_command() -> Command {
    Command::new("add")
        .description("add command")
        .alias("a")
        .usage("cli add(a) [nums...]")
        .action(add_action)
}

fn sub_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum();
    println!("{}", sum);
}

fn sub_command() -> Command {
    Command::new("sub")
        .description("sub command")
        .alias("s")
        .usage("cli sub(s) [nums...]")
        .action(sub_action)
}
fn env_command() -> Command {
    use std::path::{Path};

    Command::new("env")
        .description("env command")
        .alias("e")
        .usage("cli env")
        .action(|ctx:&Context|{
          let args: Vec<String> = env::args().collect() ;

          for(key, value) in env::vars() {
            // println!("{}={}", key, value);
          }

          let key  = "CARGO_MANIFEST_DIR";
          println!("CARGO_MANIFEST_DIR: {}", env::var(key).unwrap()) ;

          let dir = env::var(key).unwrap() ;
        //   dotenv().ok();
        let my_path = Path::new(&dir).join(".env");
        dotenv::from_path(my_path.to_str().unwrap()).ok();
        
        for(key, value) in env::vars() {
            println!("{}={}", key, value);
        }
        if Path::new(&dir).join(".env").exists() {
            println!("{} exists",".env file")
        }
        println!("DB: {}", env::var("MY_DB_URL").unwrap());

        })
}
fn config_command() -> Command {
    use std::path::{Path};
    use config::Config;
    use std::collections::HashMap;

    Command::new("config")
        .description("config command")
        .alias("c")
        .usage("cli config")
        .action(|ctx:&Context|{
          println!("config test");

          let key  = "CARGO_MANIFEST_DIR";
          let dir = env::var(key).unwrap() ;
          let settings_file = Path::new(&dir).join("config/Settings") ; // 不带后缀可以识别么？ 

          let settings = Config::builder()
          // Add in `./Settings.toml`
          .add_source(config::File::with_name(settings_file.to_str().unwrap()))
          // Add in settings from the environment (with a prefix of APP)
          // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
          .add_source(config::Environment::with_prefix("APP"))
          .build()
          .unwrap();
  
      // Print out our settings (as a HashMap)
      println!(
          "{:?}",
          settings
              .try_deserialize::<HashMap<String, String>>()
              .unwrap()
      );
          

        })
}