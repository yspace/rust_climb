pub mod command_loader;
pub mod my_module;
pub mod extensions;

use clap::Command;

use self::extensions::Extensions;

/*
let data = Data::new(Mutex::new(YOUR_DATA))

app.app_data(Data::clone(&data));

async fn index(data: Data<Mutex<YOUR_DATA_TYPE>>) -> impl Responder {

}
 */
pub trait Module {
    fn get_commands(&self) -> Vec<Command>;
}

/*
Is it possible to share app state between actors?
@see https://github.com/actix/examples/issues/74
pub struct AppState {
    pub db_addr: Arc<Addr<DbActor>>,
    pub queue_addr: Arc<Addr<QueueActor>>
}

? 这啥意思：
Use SystemRegistry to store global actors and access them anywhere.

 */
pub struct App {
    // components: Vec<Box<dyn Component>>,
    modules: Vec<Box<dyn Module>>,
    extensions: Extensions,
}
impl App {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn add(&mut self, module: impl Module + 'static) {
        self.modules.push(Box::new(module));
    }

    // 需要参考： actix_web::web::Data 类配合实现应用组件的共享问题
    pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
        self.extensions.insert(ext);
        self
    }
}

impl Module for App {
    fn get_commands(&self) -> Vec<Command> {
        let mut cmd = cli();
        for module in &self.modules {
            cmd = cmd.subcommands(module.get_commands());
        }
        vec![cmd]
    }
}

fn cli() -> Command {
    Command::new("hi_clap")
        .subcommand(Command::new("ping").about("Get a response"))
        .subcommand(Command::new("quit").alias("exit").about("Quit the REPL"))
}
