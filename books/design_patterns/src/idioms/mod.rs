use design_patterns::{ChapterNode, ContentSection};

mod borrowed_type4args ;

pub fn build_chapter(parent: &mut ChapterNode){
   

    let mut chapter_node = ChapterNode::new(
        "2".to_string(),
    "idioms".to_string());

    let mut content_section = ContentSection::new("h1 some title".to_string());
    fn my_example(){
        println!("this is idioms chapter") ;
    }
    content_section.add_example_runner(my_example) ;
    chapter_node.add_content_section(content_section);

    borrowed_type4args::build_chapter(&mut chapter_node) ;



    parent.add_chapter(chapter_node );
}

pub fn register_chapters() {

}

pub fn hi(){
    println!("in idioms") ;
}