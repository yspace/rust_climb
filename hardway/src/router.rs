use radix_trie::{Trie, TrieCommon};

// see [Async I/O in Depth: Thread Pools, Radix Trees, Channels and More - High Performance HTTP Web Servers](https://www.youtube.com/watch?v=fdxhcDne2Ww&ab_channel=ThomasHolloway%F0%9F%8F%95)


pub type HandlerFn = fn();
// pub type HandlerFn = fn(Request) -> Result<Response> ;

pub struct Router {
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

    pub fn handle(&self, path: &str) {
        self.routes.get(path).unwrap()();
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
