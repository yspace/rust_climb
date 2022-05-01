#[derive(Debug)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
pub struct QuestionId(String);
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

impl std::fmt::Display for Question{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Question {{ id: {}, title: {}, content: {} }}", self.id.0, self.title, self.content)
    }
}

#[test]
fn test_question() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
   println!("{}", question);
}
