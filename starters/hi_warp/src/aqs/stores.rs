use super::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::RwLock;

mod pg ;

#[derive(Debug, Clone)]
pub struct Store {
   pub   questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}


impl Store {
   pub fn new() -> Self {
        Self {
            // questions: HashMap::new(),
            questions: Arc::new(RwLock::new(Self::init())),

        }
    }

   

     
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json"); 
        serde_json::from_str(file).expect("can't read questions.json")
        }
         
}
