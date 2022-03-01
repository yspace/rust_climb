use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.4".to_string(),
        "The Default Trait"
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "
        rust 不能强迫每个类型都拥有new方法 所以诞生了Default \r\n
        可用与很多容器类或泛型类 如： Option::unwrap_or_default()\r\n
        不仅单元素容器 像Cow,Box或Arc 为被包裹类型实现了Default \r\n
        可以为结构 #[derive(Default)] 自动实现Dsefault 只要其字段都实现了它 \r\n
        越多类型实现Default 对其他类型就越有利。

        构造器可以有不同的名称 不同的参数。 默认构造起却无法选择 且不能有参数。
    ",
    )));
    chapter_node.add_content_section(content_section);

    
    parent.add_chapter(chapter_node);
}

mod example {
    use std::{path::PathBuf, time::Duration};
    pub fn run() {
       println!("default example") ;

       let mut conf = MyConfiguration::default();
       conf.check = true;
       println!("conf = {:#?} ", conf);

       // 仅部分采用默认值 效果同上
       let conf1 = MyConfiguration{
           check: true,
           ..Default::default()
       };

       assert_eq!(conf1,conf) ;
    }

    #[derive(Default, Debug, PartialEq)]
    struct MyConfiguration{
        // Option default to None
        output: Option<PathBuf>,
        // Vecs default to empty vector
        search_path:Vec<PathBuf>,
        // Duration default to zero time
        timeout: Duration,
        // bool default to false
        check: bool,
    }

    impl MyConfiguration{
        // add setters here
    }



}