// pub mod  async_http_request;

pub mod handlers;

// #[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    // 复杂类型 只能用Debug trait了！
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
// new type 设计模式
struct QuestionId(String);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }

    // pub fn update_title0(&mut self, title: String) -> Self {
    //     Self::new(self.id, title, self.content, self.tags)
    // }
    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.tags)
    }
}

use std::io::{Error, ErrorKind};
use std::str::FromStr;
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        // 对于字符串 可以做一些格式｜范围｜类型 ..校验 有个validate库最好
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[test]
fn test_foo() {
    let question = Question::new(
        // QuestionId("1".to_string()),
        // QuestionId::from_str("1").unwrap(),
        // 慎用unwrap expect 他们会panic！的
        QuestionId::from_str("1").expect("empty Question Id"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:#?}", question);
}
