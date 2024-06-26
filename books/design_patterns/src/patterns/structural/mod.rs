use design_patterns::{ChapterNode, ContentSection};

 

pub fn build_chapter(parent: &mut ChapterNode){
   

    let mut chapter_node = ChapterNode::new(
        "3.3".to_string(),
    "Creational Patterns".to_string(),);

    let mut content_section = ContentSection::new("h1 some title".to_string());
    fn my_example(){
        println!("this is idioms chapter") ;
    }
    content_section.add_example_runner(my_example) ;
    chapter_node.add_content_section(content_section);

   

    parent.add_chapter(chapter_node );
}
 