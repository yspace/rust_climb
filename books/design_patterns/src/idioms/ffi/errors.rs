use design_patterns::{ChapterNode, ContentSection};



pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.9.1".to_string(),
        "
        Idiommatic Errors
        "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
   // content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "
            C语言中 错误是通过return codes表示的，rust 类型系统通过一个全类型来允许更丰富的错误信息可被捕获，传播。
            This best practice shows different kinds of error codes, and how to expose them in a usable way:

1. Flat Enums should be converted to integers and returned as codes.
2. Structured Enums should be converted to an integer code with a string error message for detail.
3. Custom Error Types should become \"transparent\", with a C representation.
        ",
    )));
    chapter_node.add_content_section(content_section);

    parent.add_chapter(chapter_node);
}


mod example{
    pub fn run() {

    }

    // 胖枚举
    enum DatabaseError {
        IsReadOnly = 1, // user attempted a write operation
        IOError = 2, // user should read the C errno() for what it was
        FileCorrupted = 3, // user should run a repair tool to recover it
    }

    /*
    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            (e as i8).into()
        }
    }
    */
}

pub mod errors {
    enum DatabaseError {
        IsReadOnly,
        IOError(std::io::Error),
        FileCorrupted(String), // message describing the issue
    }
    /*

    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            match e {
                DatabaseError::IsReadOnly => 1,
                DatabaseError::IOError(_) => 2,
                DatabaseError::FileCorrupted(_) => 3,
            }
        }
    }
    */
}