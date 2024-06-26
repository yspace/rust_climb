use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
pub struct QuestionId(String);

impl Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err( Error::new(ErrorKind::InvalidInput,"no id provided")),
        }
    }
}

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }

    // fn update_title(&self)
}

// “Display is similar to Debug, but Display is for user-facing output, and so cannot be derived.”
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Question {{ id: {}, title: {}, content: {} }}",
            self.id.0, self.title, self.content
        )
    }
}

// impl std::fmt::Debug for Question {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{:?}", self.tags)
//     }
// }

#[test]
fn test_question() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{}", question);
    println!("{:?}", question);
}
