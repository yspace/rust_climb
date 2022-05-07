
use std::collections::HashMap;

use crate::models::{
    QuestionId,
    Question,
};

pub struct Store {
    pub questions: HashMap<QuestionId, Question>,

}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: HashMap::new(),
        }
    }

    // pub fn add_question(&mut self, question: Question) {
    //     self.questions.insert(question.id, question);
    // }
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
}