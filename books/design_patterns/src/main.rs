use design_patterns::{ChapterNode, ContentSection};

mod idioms ;

fn main() {
    println!("Hello, world!");

    idioms::hi();

    let mut chapter_node = ChapterNode::new(
        "0".to_string(),
    "rust patterns".to_string());

    idioms::build_chapter(&mut chapter_node) ;

   
    chapter_node.tree(0) ;
    // chapter_node.run_all() ;
    chapter_node.run_latest_chapter() ;
}
