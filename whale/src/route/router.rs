use radix_trie::{Trie, TrieCommon};

use std::error::Error;

// see [Async I/O in Depth: Thread Pools, Radix Trees, Channels and More - High Performance HTTP Web Servers](https://www.youtube.com/watch?v=fdxhcDne2Ww&ab_channel=ThomasHolloway%F0%9F%8F%95)

pub type HandlerFn = fn();
// pub type HandlerFn = fn(Request) -> Result<Response> ;

#[derive(Debug)]
pub struct Router {
    // 有机会找个替代品 poem里面有个
    routes: Trie<String, fn()>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Trie::new(),
        }
    }

    pub fn insert(&mut self, path: &str, f: HandlerFn) {
        self.routes.insert(path.to_string(), f);
    }

    pub fn descendents(&self, path: &str) -> Vec<&String> {
        let mut children = vec![];

        for (k, v) in self.routes.get_raw_ancestor(path).children().enumerate() {
            // println!("{} -> {:?} \n\n", k, v);
            //println!("  {:?} \n\n",v.keys().map(|x| x.to_string()).collect::<String>());

            for k in v.keys() {
                // println!("{}", k) ;
                children.push(k );
            }

            // if v.key().is_some() {
            //     children.push(v.key().unwrap());
            // }
        }
        children
    }

    pub fn handle(&self, path: &str) ->Result<(),String> {
        println!("<-- handle :{} \r\n" ,path);
        let handler = self.routes.get(path);
        if handler.is_some() {
           handler .unwrap()() ;
        }else{
            return Err("no path".to_string());
        }
        println!("\r\n -- {} />" ,path);
        Ok(())
    }
    // pub fn handle(&self, path: &str , req: Request) -> Result<Response> {
    //     self.routes.get(path).unwrap()(req)
    // }
}

// sample configuration
pub fn configure(r: &mut Router) {
    r.routes.insert("/".to_string(), || {
        println!("fn: /");
    });
}

mod node {
    use super::*;

    pub struct Node {
        pub nodes: Vec<Node>,
        pub key: String,
        pub handler: Option<HandlerFn>,
    }

    impl Node {
        pub fn new(key: &str) -> Self {
            Self {
                nodes: Vec::new(),
                key: key.to_string(),
                handler: None,
            }
        }
        pub fn insert(&mut self, path: &str, f: HandlerFn) {
            match path.split_once('/') {
                Some((root, "")) => {
                    self.key = String::from(root);
                    self.handler = Some(f);
                }
                Some(("", path)) => {
                    self.insert(path, f);
                }
                Some((root, path)) => {
                    let node = self.nodes.iter_mut().find(|m| root == &m.key);
                    match node {
                        Some(n) => {
                            n.insert(path, f);
                        }
                        None => {
                            let mut node = Node::new(root);
                            node.insert(path, f);

                            self.nodes.push(node);
                        }
                    }
                }
                None => {
                    let mut node = Node::new(path);
                    node.handler = Some(f);

                    self.nodes.push(node);
                }
            }
        }
    }
}

#[test]
fn test_ok() {
    let mut router = Router::new();
    router.insert("/", || {});
    router.insert("/2", || {});
    router.insert("/3", || {});

    println!("{:?}", router.descendents("/"));
}
