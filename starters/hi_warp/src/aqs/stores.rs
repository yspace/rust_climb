use super::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>,
}


impl Store {
   pub fn new() -> Self {
        Self {
            // questions: HashMap::new(),
            questions: Self::init(),
        }
    }

   pub  fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }

    fn init0(self) -> Self {
        let question = Question::new(
        QuestionId::from_str("1").expect("Id not set"),
         "How?".to_string(),
        "Please help!".to_string(), Some(vec!["general".to_string()])
        );
        self.add_question(question)
     }
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json"); 
        serde_json::from_str(file).expect("can't read questions.json")
        }
         
}
