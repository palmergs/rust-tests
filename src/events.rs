use std::cmp::Ordering;
use std::ops::Range;
use super::Alias;

#[derive(Debug)]
pub struct Event {
    pub key: String,
    pub name: String,
    pub range: Range<i64>,
    pub alias: Vec<Alias>,
    pub races: Vec<usize>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Event {
    pub fn new(key: &str, name: &str) -> Event {
        Event{
            key: key.to_string(),
            name: name.to_string(),
            range: 0..1,
            alias: Vec::new(),
            races: Vec::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl Eq for Event {}