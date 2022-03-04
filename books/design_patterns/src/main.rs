use design_patterns::{ChapterNode, ContentSection};

mod idioms ;
mod patterns ;

fn main() {
    // println!("Hello, world!");
    use std::env;

    let args: Vec<String> = env::args().collect();
    // println!("get args {:?}", args) ;
    let mut chapter = None ;
    if args.len() > 1 {
        chapter = args.get(1);
        println!("run chapter {:?}", chapter);
    }

    // idioms::hi();

    let mut chapter_node = ChapterNode::new(
        "0".to_string(),
    "rust patterns".to_string());

    idioms::build_chapter(&mut chapter_node) ;
    patterns::build_chapter(&mut chapter_node) ;

   
    chapter_node.tree(0) ;
    // chapter_node.run_all() ;
    
    match chapter {
        Some(chapter) => {
            println!("{}", chapter )  ;
           match chapter_node.run_chapter(chapter) {
               Ok(_) =>{}, 
               Err(_) => println!("can't find the chapter: {}", chapter),
           } ;
        },
        None => chapter_node.run_latest_chapter() ,
    }
    
    


  
}
