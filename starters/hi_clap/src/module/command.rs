use clap::ArgMatches;

use std::collections::HashMap;

// type HandlerFn = fn(&ArgMatches) -> Result<Vec<String>, String>;
type HandlerFn = fn(&ArgMatches) ;

pub struct Command{
    command: clap::Command,
    handler: Option<HandlerFn>,
    // children: HashMap<String, Box<Command>>,
    children: HashMap<String,  Command>,
}
impl Command {

    pub fn new(command: clap::Command,handler: Option<HandlerFn>)-> Self {

        Self{
            command,
            handler,
            children: HashMap::new(),
        }

    }

   pub fn add_child(&mut self,cmd: Command){
        self.children.insert(cmd.command.get_name().into(),cmd);
    }

    pub fn run(&self, matches: &ArgMatches){

        // matches.

        // self.handler.unwrap()(&matches);
        if let Some(handlerFn) = self.handler{
            // 有处理函数 就交给处理函数 
            handlerFn(&matches);
        }else{
            // 
            println!(" {} 命令自己不存在handler配置  找子处理...",self.command.get_name());
            if let Some(sub_command_matches) = &matches.subcommand() {
                let (sub_cmd, sub_matches) = * sub_command_matches ;
                // self.traverse_path(path)
                if self.children.contains_key(sub_cmd) {
                //    let entry = self.children.entry(sub_cmd.into());
                //     entry.into()
                    self.children.get(sub_cmd).unwrap().run(sub_matches) ;
                }

            }else{
                // 找不到么？
                println!("yiii  弄啥来！{}",self.command.get_name());
            }

        }
        
    }

    // symfony 框架中 有两个运行命令相关的实现 execute 是保护方法 供子类复写的 run是公共方法
    // fn execute(matches: &ArgMatches){
        
    // }
    
}

impl Command {
    // fn traverse_path<'p>(&mut self, mut path: &'p [Id]) -> (&mut Node, &'p [Id]) { // '
    //     if self.children.contains_key(&path[0]) {
    //         self.children[path[0]].traverse_path(path[1..])
    //     } else {
    //         (self, path)
    //     }
    // }
    fn traverse_path<'p>(&mut self, mut path: &'p [String]) -> (&mut Self, &'p [String]) { // '
        if self.children.contains_key(&path[0]) {
            let mut child = 
            // .children.entry(&path[0]);
            self.children.get_mut(&path[0]).unwrap();
            child.traverse_path(&path[1..])
        } else {
            (self, path)
        }
    }
}

struct Tree {
    root: Box<Command>
 }

 impl Tree {
    /// Traverse the nodes along the specified path.
    /// Return the node at which traversal stops either because the path is exhausted
    /// or because there are no more nodes matching the path.
    /// Also return any remaining steps in the path that did not have matching nodes.
    fn traverse_path<'p>(&mut self, mut path: &'p [String]) -> (&mut Command, &'p [String]) { // '
        self.root.traverse_path(path)
    }
}

 #[test]
 fn test_command(){

    fn handle_user(matches: &ArgMatches){
        println!("{:?}", matches);
    }

    let cmd = Command::new(
        clap::Command::new("user"),
        Some(handle_user),
    );

    let matches = cmd.command
    .clone()
    // .try_get_matches_from(["user", "--mammal", "dog"])
    .try_get_matches_from(["user",  ])
    .unwrap();

    cmd.run(&matches);


 }

 #[test]
 fn test_command_child(){

    fn handle_user_create(matches: &ArgMatches){
        println!("{:?}", matches);
    }

    let mut cmd = Command::new(
        clap::Command::new("user")
        .allow_external_subcommands(true),
       None
    );

    let sub_command = Command::new(
        clap::Command::new("create"),
       Some(handle_user_create),
    );
    cmd.add_child(
        sub_command
    );

    let matches = cmd.command
    .clone()
    // .try_get_matches_from(["user", "--mammal", "dog"])
    .try_get_matches_from(["user", "create" ])
    .unwrap();

    cmd.run(&matches);


 }