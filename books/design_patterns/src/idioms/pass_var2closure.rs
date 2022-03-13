use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.11".to_string(),
        "
        Pass variables to closure
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "
        Description
        By default, closures capture their environment by borrowing.
         Or you can use move-closure to move whole environment.
          However, often you want to move just some variables to closure, 
          give it copy of some data, pass it by reference, or perform some other transformation.
        
        Use variable rebinding in separate scope for that.
        ",
    )));
    chapter_node.add_content_section(content_section);

    
    parent.add_chapter(chapter_node);
}

mod example {
    pub fn run(){
        use std::rc::Rc ;

        let num1 = Rc::new(1) ;
        let num2 = Rc::new(2) ;
        let num3 = Rc::new(3) ;

        let closure = {
            // mum1 is moved   
            let num2 = num2.clone();
            let num3 = num3.as_ref();
            move || {
                *num1 + *num2 + *num3 ;
            }
        };
        // instead of
        /*
        let num2_cloned = num2.clone();
        let num3_borrowed = num3.as_ref();
        let closure = move || {
            *num1 + *num2_cloned + *num3_borrowed;
        };
        */
        // println!("{}{}{}",num1,num2 ,num3);
        println!("{} , {}",num2 ,num3);
    }
}