
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Field {
    i: u8,
    y: u8,
}

impl Field {
    pub fn new() -> Self {
        Self { i: 8, y: 6 }
    }
    pub fn set_i(&mut self, i: u8) {
        self.i = i;
    }
}

#[derive(Debug)]
pub struct Node<T> {
    value: RefCell<T>,
    parent: RefCell<Option<Weak<Node<T>>>>,
    child: RefCell<Option<Rc<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(Self {
            value: RefCell::new(value),
            parent: RefCell::new(None),
            child: RefCell::new(None)
        })
    }
    pub fn connect(parent: &Rc<Self>, child: &Rc<Self>) {
        parent.child.borrow_mut()
            .replace(child.clone());
        child.parent.borrow_mut()
            .replace(Rc::downgrade(child));
    }
    pub fn value(&self) -> &RefCell<T> { &self.value }
    pub fn to_string(&self) -> String
        where T: Debug {
        format!(
            "{:?} {:?}",
            *self.value.borrow(),
            self.child.borrow().as_ref()
        )
    }
}
/*
#[derive(Debug)]
pub struct Parent {
    field_1: Field,
    child: RefCell<Option<Child>>,
}

impl Parent {
    pub fn new() -> Rc<RefCell<Self>> {
        let n = Rc::new(RefCell::new(Self {
            field_1: Field::new(),
            child: RefCell::new(None),
        }));

        *n.borrow_mut().child.borrow_mut() = Some(Child::new(&n));

        n
    }

    pub fn modify(&mut self) {
        self.field_1.i = 9;
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?} {}",
            self.field_1,
            self.child.borrow().as_ref().unwrap()
        )
    }
}

#[derive(Debug)]
pub struct Child {
    parent: Weak<Parent>,
    field_2: Field,
}

impl Child {
    pub fn new(parent: &Rc<Parent>) -> Self {
        Self {
            parent: Rc::downgrade(parent),
            field_2: Field::new(),
        }
    }
}

impl fmt::Display for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.parent.upgrade().unwrap().field_1.i == 1 {
            write!(f, "set: {:?}", self.field_2)
        } else {
            write!(f, "not set {:?}", self.field_2)
        }
    }
}*/

#[test]
fn main() {
    let parent = Node::new(Field::new());
    let child = Node::new(Field::new());
    Node::connect(&parent, &child);
    parent.value()
        .borrow_mut()
        .set_i(8);
    println!("{:?}", &parent);
    println!("{}", parent.to_string());
    /*let mut parent = Parent::new();
    parent.borrow_mut().modify();
    println!("{}", parent.borrow().to_string());*/
}
