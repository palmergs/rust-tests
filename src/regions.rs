use std::cmp::Ordering;
use std::ops::Range;

use super::Alias;

#[derive(Debug)]
pub struct Region {
    pub key: String,
    pub name: String,
    pub plural: Option<String>,
    pub category: Option<String>,
    pub alias: Vec<Alias>,
    pub races: Vec<String>,
    pub range: Option<Range<i64>>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

impl Region {
    pub fn new(key: &str, name: &str) -> Region {
        Region {
            key: key.to_string(),
            name: name.to_string(),
            plural: None,
            category: None,
            alias: Vec::new(),
            races: Vec::new(),
            range: None,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn in_range(&self, year: i64) -> bool {
        match &self.range {
            Some(range) => range.start <= year && range.end >= year,
            None => true,
        }
    }

    pub fn has_race(&self, key: &str) -> bool {
        if self.races.len() == 0 {
            return true;
        }
        for r in &self.races {
            if r == key {
                return true;
            }
        }
        false
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Region {}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
