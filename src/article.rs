use sorted_vec::SortedVec;

use std::i32::{ MIN, MAX };
use std::cmp::{ Ordering };

pub struct Region {
    id: String,
    name: Option<String>,
    plural: Option<String>,
    alias: Vec<Alias>,
    parent: Option<String>,
    children: Vec<String>,
}

pub struct GeoFeature {
    id: String,
    name: String,
}

pub enum Tone {
    Positive,
    Neutral,
    Negative,
}

pub struct Alias {
    name: String,
    tone: Tone,
    races: Vec<String>,
}

pub struct Timeline {
    eras: SortedVec<Era>,
    events: SortedVec<Event>, 
}

pub struct Era {
    id: String,
    abbr: String,
    from: Option<i32>,
    to: Option<i32>,
    races: Vec<String>
}

pub struct Event {
    id: String,
    name: Option<String>,
    from: Option<i32>,
    to: Option<i32>,
    alias: Vec<Alias>,
    races: Vec<String>,
    parent: Option<String>,
    children: Vec<String>,
}

impl Event {
    pub fn start(&self) -> i32 {
        match self.from {
            Some(n) => n,
            None => std::i32::MIN,
        }
    }

    pub fn end(&self) -> i32 {
        match self.to {
            Some(n) => n,
            None => std::i32::MAX,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start().cmp(&other.start())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Event {}
