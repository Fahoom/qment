use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct Question {
    groups: HashMap<String, IndexSet<String>>,
    sections: IndexMap<String, Section>,
}

impl Question {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            sections: IndexMap::new(),
        }
    }

    pub fn groups_mut(&mut self) -> hash_map::IterMut<String, IndexSet<String>> {
        self.groups.iter_mut()
    }

    pub fn add_group(&mut self, name: &str) {
        self.groups.insert(name.into(), IndexSet::new());
    }

    pub fn rename_group(&mut self, old_name: &str, new_name: &str) -> Option<()> {
        let old = self.groups.remove(old_name);
        if let Some(hashset) = old {
            self.groups.insert(new_name.into(), hashset);
            Some(())
        } else {
            None
        }
    }

    pub fn remove_group(&mut self, name: &str) {
        self.groups.remove(name);
    }

    pub fn has_group(&mut self, name: &str) -> bool {
        self.groups.contains_key(name)
    }

    pub fn sections(&self) -> indexmap::map::Iter<String, Section> {
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
        let mut sections = IndexMap::new();

        sections.insert("Question".into(), Section::default());
        sections.insert("Mark Scheme".into(), Section::default());

        Self {
            groups: Default::default(),
            sections,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Section {
    pub text: Option<String>,
}
