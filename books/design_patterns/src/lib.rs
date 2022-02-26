use std::{cell::RefCell, rc::Rc};

// https://self-publishingschool.com/parts-of-a-book/
pub struct ChapterNode{
    chapter_number: String ,
    chapter_title: String ,
    page_contents: Vec<ContentSection>,

    children: Vec<ChapterNode>,
}

impl ChapterNode{
    pub fn new(chapter_number: String, chapter_title: String) -> Self {
        Self{
            chapter_number ,
            chapter_title,
            page_contents: vec![],

            children: Vec::new(),
        }
    }

    pub fn add_content_section(&mut self, content_section: ContentSection){
        self.page_contents.push(content_section) ;
    }

    pub fn run_all(&mut self){

        println!("\n === run ch:{} =》 {} ===", self.chapter_number, self.chapter_title) ;
        println!("run page_contents \r\n \r\n ");

        for mut content_section in self.page_contents.clone() {
            content_section.run() ;
        }
        println!("\n === end ch:{} === \n", self.chapter_number) ;
    }
}
#[derive( Clone)]
pub struct ContentSection{
    title: String ,
    example_runners: CallbacksMut,
    // example_runners: Vec<CallbacksMut>,
    // sections: Vec<ContentSection>,
}

impl ContentSection{
    pub fn new(title: String) -> Self {
        Self{
            title,
            // example_runners: Vec::new(),
            example_runners: CallbacksMut::new(),
        }
    }

    pub fn add_example_runner<F: FnMut()+'static>(&mut self, callback: F){
        self.example_runners.register(callback) ;
    }

    pub fn run(&mut self) {
        self.example_runners.call();
    }
}

#[derive(Clone)]
struct CallbacksMut {
    callbacks: Vec<Rc<RefCell<FnMut()>>>,
}

impl CallbacksMut {
    pub fn new() -> Self {
        CallbacksMut { callbacks: Vec::new() }
    }

    pub fn register<F: FnMut()+'static>(&mut self, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.push(cell);
    }

    pub fn call(&mut self) {
        for callback in self.callbacks.iter() {
            let mut closure = callback.borrow_mut();
            (&mut *closure)();
        }
    }
}

fn demo_mut(c: &mut CallbacksMut) {
    c.register(|| println!("Callback 1:"));
    c.call();

    {
        let mut count: usize = 0;
        c.register(move | | {
            count = count+1;
            println!("Callback 2:  ({}. time)",  count);
        } );
    }
    c.call(); c.clone().call();
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let mut chapter_node = ChapterNode::new(
            "1".to_string(),
        "chapter 1".to_string());

        let mut content_section = ContentSection::new("h1 some title".to_string());
        fn my_example(){
            println!("this is my example") ;
        }
        content_section.add_example_runner(my_example) ;
        chapter_node.add_content_section(content_section);
        chapter_node.run_all() ;
    }

}