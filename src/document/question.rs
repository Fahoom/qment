use std::collections::{HashMap, hash_map, HashSet};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Question {
    groups: HashMap<String, HashSet<String>>,
    sections: HashMap<String, Section>
}

impl Question {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            sections: HashMap::new()
        }
    }

    pub fn groups_mut(&mut self) -> hash_map::IterMut<String, HashSet<String>> {
        self.groups.iter_mut()
    }
    
    pub fn add_group(&mut self, name: &str) {
        self.groups.insert(name.into(), HashSet::new());
    }

    pub fn remove_group(&mut self, name: &str) {
        self.groups.remove(name);
    }

    pub fn has_group(&mut self, name: &str) {
        self.groups.contains_key(name);
    }

    pub fn sections(&self) -> hash_map::Iter<String, Section> {
        self.sections.iter()
    }

    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections.get_mut(name)
    }

    pub fn add_section(&mut self, name: &str) {
        self.sections.insert(name.into(), Section::default());
    }

    pub fn remove_section(&mut self, name: &str) {
        self.sections.remove(name);
    }
}

impl Default for Question {
    fn default() -> Self {
        let mut sections = HashMap::new();

        sections.insert("Question".into(), Section::default());
        sections.insert("Mark Scheme".into(), Section::default());

        Self { groups: Default::default(), sections }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Section {
    text: Option<String>
}