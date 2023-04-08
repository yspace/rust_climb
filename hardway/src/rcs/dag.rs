// directed acyclic graph

use std::rc::Rc;

struct Node{
    value: &'static str,
    edges: Vec< Rc<Node>>,
}

impl Node {
    fn display(&self)   {
        println!("value: {}", self.value);

        for edge in &self.edges{
            edge.display();
        }

       println!("Up");  
    }
}

impl Drop for Node {
    fn drop(&mut self) {
       println!("Dropping {}", self.value);
    }
}

#[test]
fn test_it(){
    let e = Rc::new(Node{
        value: "e",
        edges: vec![],
    });
    let d = Rc::new(Node{
        value: "d",
        edges: vec![e.clone()],
    });
    let a = Node{
        value: "a",
        edges: vec![
            Rc::new( Node{
                value: "b",
                edges: vec![d.clone()],
            }),
            Rc::new(Node{
                value: "c",
                edges: vec![d.clone(),e.clone()],
            }),

            d.clone(),
            e,
        ],
    };

    // 弱引用
    let d_weak = Rc::downgrade(&d);
    drop(d);


    a.display();
    // drop(a);

    // let d = d_weak.upgrade();
    if let Some(d) = d_weak.upgrade(){
        println!("recove d");
        d.display();
    }else{
        println!("d is dropped!");
    }
    drop(a);
}