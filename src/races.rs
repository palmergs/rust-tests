use super::Alias;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Race {
    pub key: String,
    pub name: String,
    pub plural: Option<String>,
    pub alias: Vec<Alias>,
    pub regions: Vec<String>,
}

impl Race {
    pub fn new(key: &str, name: &str) -> Race {
        Race {
            key: key.to_string(),
            name: name.to_string(),
            plural: None,
            alias: Vec::new(),
            regions: Vec::new(),
        }
    }

    pub fn pc() -> Vec<&'static str> {
        vec![
            "human", "elf", "dwarf", "rulligg", "feletaur", "centaur", "urunai", "gobru", "urg",
        ]
    }
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Race {}

impl Ord for Race {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Race {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
