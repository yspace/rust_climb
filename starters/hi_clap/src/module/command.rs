use clap::ArgMatches;

use std::collections::HashMap;

type HandlerFn = fn(&ArgMatches) -> Result<Vec<String>, String>;

struct Command{
    command: clap::Command,
    handler: Option<HandlerFn>,
    // children: HashMap<String, Box<Command>>,
    children: HashMap<String,  Command>,
}
impl Command {

    fn add_child(&mut self,cmd: Command){
        self.children.insert(cmd.command.get_name().into(),cmd);
    }
    
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

 