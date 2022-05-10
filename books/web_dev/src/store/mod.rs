
use std::collections::HashMap;

use crate::models::{
    QuestionId,
    Question,
};

#[derive(Debug, Clone)]
pub struct Store {
    pub questions: HashMap<QuestionId, Question>,

}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Self::init(),
            // questions: HashMap::new(),
        }
    }

    pub fn init() -> HashMap<QuestionId,Question> {
        // let file = include_str!("../../questions.json");
        let file = include_str!("./questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
         
    }

    // pub fn add_question(&mut self, question: Question) {
    //     self.questions.insert(question.id, question);
    // }
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
}