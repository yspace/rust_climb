use design_patterns::{ChapterNode, ContentSection};

pub fn build_chapter(parent: &mut ChapterNode) {
    let mut chapter_node = ChapterNode::new(
        "2.3".to_string(),
        "Constructors
    "
        .to_string(),
    );

    let mut content_section = ContentSection::new("h1 some title".to_string());
    content_section.add_example_runner(example::run);
    content_section.set_description(Some(String::from(
        "
        rust 中没有语言内置的类型构造器 ，但惯例上 经常使用new关联函数来构造对象
    ",
    )));
    chapter_node.add_content_section(content_section);

    let mut content_section = ContentSection::new("Default Constructors".to_string());
    content_section.add_example_runner(||{ 
        example_default_constructor::run();
        derived_default::run();
    });
    content_section.set_description(Some(String::from(
        "
        rust 通过Default trait 来支持默认构造起
    ",
    )));
    chapter_node.add_content_section(content_section);

    parent.add_chapter(chapter_node);
}

mod example {
    pub fn run() {
        println!("constructor in rust");

        let s = Second::new(42);
        assert_eq!(42, s.value());
    }

    pub struct Second {
        value: u64,
    }

    impl Second {
        // Constructs a new instance of [`Second`].
        // Note this is an associated function - no self.
        pub fn new(value: u64) -> Self {
            Self { value }
        }

        /// Returns the value in seconds
        pub fn value(&self) -> u64 {
            self.value
        }
    }
}

mod example_default_constructor {

    pub fn run() {
        println!("default constructor  ");
        let s = Second::default();
        assert_eq!(0, s.value());
    }
    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let s = Second::default();
    /// assert_eq!(0, s.value());
    /// ```
    pub struct Second {
        value: u64,
    }

    impl Second {
        /// Returns the value in seconds.
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    impl Default for Second {
        fn default() -> Self {
            Self { value: 0 }
        }
    }
}

mod derived_default {

    pub fn run(){
        let s = Second::default();
        println!("value of second: {}",s.value());
    }

    /// Time in seconds.
///
/// # Example
///
/// ```
/// let s = Second::default();
/// assert_eq!(0, s.value());
/// ```
#[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}
}