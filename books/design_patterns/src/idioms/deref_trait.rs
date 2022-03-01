use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.5".to_string(),
        "Collections are smart pointers
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "
            用Deref trait 来对待集合如同灵巧指针那样，提供数据的owing 和 borrowed 视图。\r\n
            Vec的deref实现，允许&Vec<T> 到&[T]的解引用 \r\n
            很多我们期望Vec实现的方法 实际上是被切片实现了
            可以参考下 String 和 &str

            动机：rust语言中 所有权与借用是关键方面。数据结构必须适当考虑这些语义以便提供好的用户体验.\r\n
            当实现一个拥有数据的数据结构时，考虑到更灵活的api应提供一个数据的借用视图
            
            优点：很多方法只能对借用视图实现，之后他们对owning view 隐式可用。
            给客户一个在 借用和获取数据所有权 之间的选择

            缺点：方法和traits只有通过解引用才可用是没有考虑bounds checking的，在带数据结构的泛型编程中使用
            此种模式会使事情变得更复杂。
        ",
    )));
    chapter_node.add_content_section(content_section);

    
    parent.add_chapter(chapter_node);
}

mod example {
    use std::ops::Deref;

    pub fn run() {
      let v = vec![1, 2, 3] ;
      is_borrowed_collection(&v) ;

      let s = String::from("some string");
      is_str_slice(&s) ;
    }

   fn is_borrowed_collection<T>(v: &[T]) {

   }
   fn is_str_slice(_ : &str){

   }


   struct Foo<T>{
       data: T ,
   }

   struct Bar<T>{
       data: T ,
   }

   impl<T> Deref for Foo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target{
       //&Box::new( Bar{data: self.data})
       &self.data
    }
   }
}