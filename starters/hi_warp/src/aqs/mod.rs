// pub mod  async_http_request;
use serde::{Serialize, Deserialize};

pub mod handlers;
pub mod stores;
pub mod errors;
// 领域类型 或者也可以选择名称：domains｜models ；可以直接重导出到根领域下 一层层引用太邋遢
// 实际上 有的目录规划时 根领域对象是被本领域其他层共同引用的 所以直接出现在根名空间下比较方便
pub mod types; 
// pub use   types::* ;


// #[derive(Debug)]
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    // 复杂类型 只能用Debug trait了！
    tags: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize,Debug, Clone, PartialEq,Eq, Hash)]
// new type 设计模式
pub struct QuestionId(String);

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

// impl std::fmt::Debug for Question {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{:?}", self.tags)
//     }
// }

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
