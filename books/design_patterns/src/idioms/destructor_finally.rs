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
        析构器：退出前必定运行的代码
         
        动机： 如果函数有多个返回点 。 在退出时执行代码就比较困难了 或者重复执行 。
        特别是在使用宏隐性返回时。 问号? 作为一种异常处理机制 如果是Err就return ，
        Ok则继续进行。不似java 没办法调度代码同时运行在正常和异常情况。Panicking
        也会过早的退出函数。

        优点：
        析构器中的代码几乎总会被执行 -- 应对异常 过早返回等情形

        缺点：
        并非一定会执行，如函数中出现无限循环 退出前crash Destructors are also not run in the case of a panic in an already panicking thread. Therefore, 
        destructors cannot be relied on as finalizers where it is absolutely essential that finalisation happens.
        ",
    )));
    chapter_node.add_content_section(content_section);

    
    parent.add_chapter(chapter_node);
}

mod example{
    pub fn run() {
        println!("in destructor_finally mod");

        bar().unwrap();
    }

    fn bar() -> Result<(), ()> {
        // These don't need to be defined inside the function.
        struct Foo;
    
        // Implement a destructor for Foo.
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("exit foo");
            }
        }
    
        // The dtor of _exit will run however the function `bar` is exited.
        let _exit = Foo;
        // Implicit return with `?` operator.
        // baz()?;
        // Normal return.
        // 
        // panic!("hi") ;
        Ok(())
    }
}