use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Struct allowing for default tags and tag-groups to be defined by the user
#[derive(Serialize, Deserialize, Default)]
pub struct Preset {
    pub tags: HashMap<String, Vec<String>>,
    pub groups: Vec<String>,
}
