use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.7".to_string(),
        "mem::{take(_), replace(_)} to keep owned values in changed enums
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "

        ",
    )));
    chapter_node.add_content_section(content_section);

    
    parent.add_chapter(chapter_node);
}

mod example{
    pub fn run(){

        let mut e = MyEnum::A{
            name:"some string".to_string(),x:0
        };
        a_to_b(&mut e) ;

        println!("{:?}",e) ;
    }

    use std::mem ;

    #[derive(Debug)]
    enum MyEnum{
        A{name: String, x:u8},
        B{name: String} ,
    }

    fn a_to_b(e: &mut MyEnum){
        if let MyEnum::A {name, x:0}
        = e{
             // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).
            *e = MyEnum::B{ name: mem::take(name)}
        }
    }

    enum MultiVariateEnum {
        A { name: String },
        B { name: String },
        C,
        D
    }
    
    fn swizzle(e: &mut MultiVariateEnum) {
        use MultiVariateEnum::*;
        *e = match e {
            // Ownership rules do not allow taking `name` by value, but we cannot
            // take the value out of a mutable reference, unless we replace it:
            A { name } => B { name: mem::take(name) },
            B { name } => A { name: mem::take(name) },
            C => D,
            D => C
        }
    }
}