use std::cmp::Ordering;
use super::Alias;

#[derive(Debug)]
pub struct Region {
    pub key: String,
    pub name: String,
    pub plural: Option<String>,
    pub alias: Vec<Alias>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Region {
    pub fn new(key: &str, name: &str) -> Region {
        Region {
            key: key.to_string(),
            name: name.to_string(),
            plural: None,
            alias: Vec::new(),
            parent: None,
            children: Vec::new()
        }
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl Eq for Region {}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering { self.name.cmp(&other.name) }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}