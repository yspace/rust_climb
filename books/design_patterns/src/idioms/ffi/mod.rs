use design_patterns::{ChapterNode, ContentSection};

mod errors ;


pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.9".to_string(),
        "
        FFI Idioms
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
   // content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "

        ",
    )));
    chapter_node.add_content_section(content_section);

    // 添加子节点
    errors::build_chapter(&mut chapter_node) ;

    parent.add_chapter(chapter_node);
}
