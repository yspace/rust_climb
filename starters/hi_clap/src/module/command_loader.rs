

struct CommandLoader{

}

// use super::router::HandlerFn;
// pub type HandlerFn = Pin<Box<dyn Fn(TcpClient) -> LocalBoxedFuture<'static, Result<()>>>>;
pub type HandlerFn = std::pin::Pin<Box<dyn Fn()>> ;

// #[derive(Debug)]
pub struct Node {
    pub nodes: Vec<Node>,
    pub key: String,
    pub handler: Option<HandlerFn>,
}

impl Node {
    pub fn new(key: &str) -> Self {
        Node {
            nodes: Vec::new(),
            key: String::from(key),
            handler: None,
        }
    }

    pub fn insert(&mut self, path: &str, f: HandlerFn) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(root);
                self.handler = Some(f);
            }
            Some(("", path)) => self.insert(path, f),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|m| root == &m.key);
                match node {
                    Some(n) => n.insert(path, f),
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

    pub fn get(&self, path: &str) -> Option<&HandlerFn> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key {
                    self.handler.as_ref()
                } else {
                    None
                }
            }
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.nodes.iter().find(|m| root == &m.key);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.nodes.iter().find(|m| path == &m.key);
                if let Some(node) = node {
                    node.handler.as_ref()
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn test_insert_routes() {
    let mut root = Node::new("");
    // root.insert("/", Box::pin(|| Ok(())));
    root.insert("/", Box::pin(|| ()));
    root.insert("/foo", Box::pin(|| println!("foo")));
    // root.insert("/foo", |_| Ok(()));
    // root.insert("/foo/bar", |_| Ok(()));
    // println!("{:?}", root);
    let my_fn  = root.get("/foo");
    assert!(my_fn.is_some());
    let result = my_fn.unwrap()(); 
    assert_eq!(result, ());
}