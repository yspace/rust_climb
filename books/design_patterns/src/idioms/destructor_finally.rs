use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.6".to_string(),
        "Finalisation in destructors
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
    pub fn run() {
        println!("in destructor_finally mod")
    }
}