extern crate regex;
use regex::Regex;

use yaml_rust::{ YamlLoader, Yaml };
use sorted_vec::SortedVec;

use std::i32::{ MIN, MAX };
use std::cmp::{ min, max, Ordering };
use std::collections::hash_map::HashMap;

pub struct Caerlun<'a> {
    id_key: Yaml,
    name_key: Yaml,
    alias_key: Yaml,
    parent_key: Yaml,
    race_key: Yaml,
    tone_key: Yaml,
    year_key: Yaml,

    timeline: Timeline<'a>,
    races: HashMap<String, &'a Race>,
    regions: HashMap<String, &'a Region>,
    events: HashMap<String, &'a Event>, 
    features: HashMap<String, &'a GeoFeature>,
}

impl<'a> Caerlun<'a> {
    pub fn new() -> Caerlun<'a> {
        Caerlun {
            id_key: Yaml::from_str("id"),
            name_key: Yaml::from_str("name"),
            alias_key: Yaml::from_str("alias"),
            parent_key: Yaml::from_str("parent"),
            race_key: Yaml::from_str("race"),
            tone_key: Yaml::from_str("tone"),
            year_key: Yaml::from_str("year"),

            timeline: Timeline::new(),
            races: HashMap::new(),
            regions: HashMap::new(),
            events: HashMap::new(),
            features: HashMap::new(),
        }
    }
}

pub struct Timeline<'a> {
    eras: SortedVec<&'a Era>,
    events: SortedVec<&'a Event>, 
}

impl<'a> Timeline<'a> {
    pub fn new() -> Timeline<'a> {
        Timeline {
            eras: SortedVec::new(),
            events: SortedVec::new(),
        }
    }
}


pub struct Race {
    id: String,
    name: String,
    plural: Option<String>,
    alias: Vec<Alias>,
}

impl Race {
    pub fn build(yaml: &Yaml) {
    }

    pub fn id(&self) -> &String { &self.id }
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for Race {}

impl Ord for Race {
    fn cmp(&self, other: &Self) -> Ordering { self.name.cmp(&other.name) }
}

impl PartialOrd for Race {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TimeRange {
    start: Option<i32>,
    end: Option<i32>,
}

impl TimeRange {
    pub fn new(time: &str) -> TimeRange {
        lazy_static! {
            static ref RANGE: Regex = Regex::new(r"\s*(to|before|until|after)\s*").unwrap();
            static ref NUMBER: Regex = Regex::new(r"\s*([-]?[0-9]+)\s*").unwrap();
        }
        match RANGE.captures(time) {
            Some(capture) => {
                let nums: Vec<&str> = NUMBER.find_iter(time).map(|mat| mat.as_str().trim()).collect();
                let one: i32 = nums[0].parse().unwrap();
                match capture.get(1).unwrap().as_str() {
                    "to" => {
                        let two: i32 = nums[1].parse().unwrap();
                        TimeRange{ start: Some(std::cmp::min(one, two)), end: Some(std::cmp::max(one, two)) }
                    },
                    "before" | "until" => TimeRange{ start: None, end: Some(one) },
                    "after" => TimeRange{ start: Some(one), end: None },
                    _ => panic!("Unable to parse time range with {}", time),
                }
            },
            None => {
                let year = time.to_string();
                let year = year.trim();
                let year: i32 = year.parse().unwrap();
                TimeRange{ start: Some(year), end: Some(year) }
            }
        }
    }
}

pub struct Era {
    id: String,
    name: String,
    range: TimeRange,
    races: Vec<String>
}

impl Era {
    fn new(id: &str, name: &str) -> Era {
        Era { id: id.to_string(), name: name.to_string(), range: TimeRange::new("1980 to 1990"), races: Vec::new() }
    }

    fn id(&self) -> &String { &self.id }
}

impl Ord for Era {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.cmp(&other.range)
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
    range: TimeRange,
    alias: Vec<Alias>,
    races: Vec<String>,
    parent: Option<String>,
    children: Vec<String>,
}

impl Event {
    fn id(&self) -> &String { &self.id }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.cmp(&other.range)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_range_from_string() {
        let range = TimeRange::new("1234");
        assert_eq!(1234 as i32, range.start.unwrap());
        assert_eq!(1234 as i32, range.end.unwrap());

        let range = TimeRange::new(" -432  ");
        assert_eq!(-432 as i32, range.start.unwrap());
        assert_eq!(-432 as i32, range.end.unwrap());

        let range = TimeRange::new("-100 to 200");
        assert_eq!(-100 as i32, range.start.unwrap());
        assert_eq!(200 as i32, range.end.unwrap());

        let range = TimeRange::new("after 1000");
        assert_eq!(1000 as i32, range.start.unwrap());
        assert_eq!(None, range.end);

        let range = TimeRange::new("before 888");
        assert_eq!(None, range.start);
        assert_eq!(888, range.end.unwrap());
    }
}
