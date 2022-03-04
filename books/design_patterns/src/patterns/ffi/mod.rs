use design_patterns::{ChapterNode, ContentSection};

 

pub fn build_chapter(parent: &mut ChapterNode){
   

    let mut chapter_node = ChapterNode::new(
        "3.4".to_string(),
    "FFI Patterns".to_string(),);

    let mut content_section = ContentSection::new("h1 some title".to_string());
    let  my_example = || {
        println!("this is {} chapter","3.4");
    };
    content_section.add_example_runner(my_example) ;
    chapter_node.add_content_section(content_section);

   

    parent.add_chapter(chapter_node );
}
 