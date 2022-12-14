use self::question::Question;
use serde::{Deserialize, Serialize};
use std::collections::{btree_map, BTreeMap};

pub mod preset;
pub mod question;

#[derive(Serialize, Deserialize)]
pub struct Document {
    questions: BTreeMap<u32, Question>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            questions: BTreeMap::new(),
        }
    }

    pub fn questions(&self) -> btree_map::Iter<u32, Question> {
        self.questions.iter()
    }

    pub fn get_question_mut(&mut self, name: u32) -> Option<&mut Question> {
        self.questions.get_mut(&name)
    }

    pub fn add_question(&mut self, name: u32) {
        self.questions.insert(name, Question::default());
    }

    pub fn add_question_with(&mut self, name: u32, question: Question) {
        self.questions.insert(name, question);
    }

    pub fn remove_question(&mut self, name: u32) {
        self.questions.remove(&name);
    }
}
