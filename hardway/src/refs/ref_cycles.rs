use std::{rc::{Weak, Rc}, cell::RefCell}; // 有两个Weak sync包里还有一个！

pub struct Node {
    value: i32,
    parent: WeakNodePtr,
    // 左右树结构中 类型是Option<NodePtr>,
    children: Vec<NodePtr>,
    // Rc<RefCell<Vec<T>>> 套太多了！😄
    // children: RefCell<Vec<NodePtr>>,
}

type WeakNodePtr = Weak<RefCell<Node>>;
type NodePtr = Rc<RefCell<Node>>;

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            parent: Weak::new(),
            children: Vec::new(),
        }
    }
}

impl From<Node> for Option<NodePtr> {
    fn from(node: Node) -> Option<NodePtr> {
        Some(Rc::new(RefCell::new(node)))
    }
}

mod  _2{
    use std::{rc::{Weak, Rc}, cell::RefCell}; 
    struct Node{
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
}