use sorted_vec::SortedVec;

use std::i32::{ MIN, MAX };
use std::cmp::{ Ordering };

pub struct Region {
    id: String,
    name: String,
    plural: Option<String>,
    alias: Vec<Alias>,
    parent: Option<String>,
    children: Vec<String>,
}

impl Region {
    pub fn new(id: &str, name: &str) -> Region {
        Region {
            id: id.to_string(),
            name: name.to_string(),
            plural: None,
            alias: Vec::new(),
            parent: None,
            children: Vec::new()
        }
    }

    pub fn id(&self) -> &String { &self.id }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
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

pub struct GeoFeature {
    id: String,
    name: String,
}

impl GeoFeature {
    pub fn new(id: &str, name: &str) -> GeoFeature { 
        GeoFeature { id: id.to_string(), name: name.to_string() }
    }
}

impl PartialEq for GeoFeature {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for GeoFeature {}

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
    name: String,
    from: Option<i32>,
    to: Option<i32>,
    races: Vec<String>
}

impl Era {
    fn new(id: &str, name: &str) -> Era {
        Era { id: id.to_string(), name: name.to_string(), from: None, to: None, races: Vec::new() }
    }

    fn id(&self) -> &String { &self.id }

    fn start(&self) -> i32 {
        match self.from {
            Some(n) => n,
            None => std::i32::MIN,
        }
    }

    fn end(&self) -> i32 {
        match self.to {
            Some(n) => n,
            None => std::i32::MAX,
        }
    }
}

impl Ord for Era {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.start().cmp(&other.start());
        if ord == Ordering::Equal {
            (self.end() - self.start()).cmp(&(other.end() - other.start()))
        } else {
            ord
        }
    }
}

impl PartialOrd for Era {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Era {
    fn eq(&self, other: &Self) -> bool { self.id() == other.id() }
}

impl Eq for Era {}

pub struct Event {
    id: String,
    name: String,
    from: Option<i32>,
    to: Option<i32>,
    alias: Vec<Alias>,
    races: Vec<String>,
    parent: Option<String>,
    children: Vec<String>,
}

impl Event {
    fn id(&self) -> &String { &self.id }

    fn start(&self) -> i32 {
        match self.from {
            Some(n) => n,
            None => std::i32::MIN,
        }
    }

    fn end(&self) -> i32 {
        match self.to {
            Some(n) => n,
            None => std::i32::MAX,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.start().cmp(&other.start());
        if ord == Ordering::Equal {
            (self.end() - self.start()).cmp(&(other.end() - other.start()))
        } else {
            ord
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool { self.id() == other.id() }
}

impl Eq for Event {}
