use design_patterns::{ChapterNode, ContentSection};


pub fn build_chapter(parent: &mut ChapterNode){
    

    let mut chapter_node = ChapterNode::new(
        "2.2".to_string(),
    "Concatenating strings with format!
    ".to_string()
);

    let mut content_section = ContentSection::new("h1 some title".to_string());
    fn my_example(){
        example::run() ;
    }
    content_section.add_example_runner(my_example) ;
    content_section.set_description(Some(String::from("
     在 mut String上使用push 或者push_str方法来构建字符串是可以的 或者使用+操作符。\r\n
     然而，使用format宏经常是更方便的选择。特别是在同时有字符串字面量和非字面量的情形

    ")));
    chapter_node.add_content_section(content_section);

    


    parent.add_chapter(chapter_node );
}


mod example{
    pub fn run() {
       println!("{}",say_hello("rust"))  ;
    }

        
    fn say_hello(name: &str) -> String {
        // We could construct the result string manually.
        // let mut result = "Hello ".to_owned();
        // result.push_str(name);
        // result.push('!');
        // result

        // But using format! is better.
        format!("Hello {}!", name)
    }
}