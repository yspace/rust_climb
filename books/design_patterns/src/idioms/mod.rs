use design_patterns::{ChapterNode, ContentSection};

mod borrowed_type4args ;
mod  concat_format;
mod  constructor;
mod  default_trait;
mod  deref_trait;
mod  destructor_finally;
mod  mem_replace ;
mod  on_stack_dyn_dispatch ;
// 
mod ffi ;
mod pass_var2closure ;

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
    concat_format::build_chapter(&mut chapter_node) ;
    constructor::build_chapter(&mut chapter_node) ;
    default_trait::build_chapter(&mut chapter_node) ;
    destructor_finally::build_chapter(&mut chapter_node) ;
    mem_replace::build_chapter(&mut chapter_node) ;
    on_stack_dyn_dispatch::build_chapter(&mut chapter_node) ;

    ffi::build_chapter(&mut chapter_node) ;
    pass_var2closure::build_chapter(&mut chapter_node) ;

    parent.add_chapter(chapter_node );
}

pub fn register_chapters() {

}

pub fn hi(){
    println!("in idioms") ;
}