extern crate regex;
use regex::Regex;

use yaml_rust::{ YamlLoader, Yaml };
use sorted_vec::SortedVec;

use std::i32::{ MIN, MAX };
use std::cmp::{ min, max, Ordering };
use std::collections::hash_map::HashMap;
use std::str::FromStr;

pub struct Caerlun<'a> {
    pub id_key: Yaml,
    pub name_key: Yaml,
    pub plural_key: Yaml,
    pub alias_key: Yaml,
    pub parent_key: Yaml,
    pub race_key: Yaml,
    pub tone_key: Yaml,
    pub year_key: Yaml,

    timeline: Timeline<'a>,
    races: HashMap<String, Race>,
    regions: HashMap<String, Region>,
    events: HashMap<&'a String, &'a Event>, 
    features: HashMap<&'a String, &'a GeoFeature>,
}

impl<'a> Caerlun<'a> {
    pub fn new() -> Caerlun<'a> {
        Caerlun {
            id_key: Yaml::from_str("id"),
            name_key: Yaml::from_str("name"),
            plural_key: Yaml::from_str("plural"),
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

    fn optional_string(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::String(s) => Some(s.to_string()),
            _ => None
        }
    }

    fn strings(&self, yaml: &Yaml) -> Vec<String> {
        match yaml {
            Yaml::Array(arr) => arr.iter().map(|s| s.as_str().unwrap().to_string()).collect(),
            _ => Vec::new()
        }
    }

    fn build_aliases(&self, yaml: &Yaml) -> Vec<Alias> {
        match yaml {
            Yaml::Array(arr) => {
                let mut vec = Vec::new();
                for a in arr {
                    match self.build_alias(a) {
                        Some(alias) => vec.push(alias),
                        None => ()
                    }
                }
                vec
            },
            _ => Vec::new()
        }
    }

    fn build_alias(&self, yaml: &Yaml) -> Option<Alias> {
        match yaml {
            Yaml::Hash(h) => {
                let tone = match h[&self.tone_key].as_str() {
                    Some(s) => Tone::from_str(s).unwrap(),
                    None => Tone::Neutral,
                };
                let alias = Alias{
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    tone: tone,
                    races: self.strings(&h[&self.race_key]),
                };
                Some(alias)
            },
            _ => None
        }
    }    

    pub fn append_race(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let id = h[&self.id_key].as_str().unwrap().to_string();
                let r = Race{
                    id: id.clone(),
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    plural: self.optional_string(&h[&self.plural_key]),
                    alias: self.build_aliases(&h[&self.alias_key]),
                };
                self.races.insert(id, r);
            },
            _ => panic!("Expected to build race instance from hash"),
        }
    }

    pub fn append_geo(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {

            },
            _ => panic!("Expected to build a geo instance from hash"),
        }
    }

    pub fn append_region(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let id = h[&self.id_key].as_str().unwrap().to_string();
                let opt_parent = self.optional_string(&h[&self.parent_key]);
                let r = Region{
                    id: id.clone(),
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    plural: self.optional_string(&h[&self.plural_key]),
                    alias: self.build_aliases(&h[&self.alias_key]),
                    parent: opt_parent.clone(),
                    children: Vec::new()
                };
                self.regions.insert(id.clone(), r);
                match opt_parent {
                    Some(parent_id) => {
                        if let Some(parent) = self.regions.get_mut(&parent_id) {
                            parent.children.push(id.clone());
                        }
                    },
                    None => (),
                }
            },
            _ => panic!("Expected to build a region instance from hash"),
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

impl FromStr for Tone {
    type Err = ();

    fn from_str(s: &str) -> Result<Tone, ()> {
        match s {
            "positive" => Ok(Tone::Positive),
            "neutral" => Ok(Tone::Neutral),
            "negative" => Ok(Tone::Negative),
            _ => Ok(Tone::Neutral),
        }
    }
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

    // Can be built from `### to ###`, `before ###`, `after ###` or `until ###`
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
