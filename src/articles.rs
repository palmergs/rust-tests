extern crate regex;
use regex::Regex;

use yaml_rust::{ YamlLoader, Yaml };
use sorted_vec::SortedVec;
use indexmap::IndexMap;

//use std::i32::{ MIN, MAX };
use std::cmp::Ordering;
use std::str::FromStr;

use super::Asset;

#[derive(Debug)]
pub struct Caerlun<'a> {
    pub id_key: Yaml,
    pub name_key: Yaml,
    pub plural_key: Yaml,
    pub alias_key: Yaml,
    pub parent_key: Yaml,
    pub race_key: Yaml,
    pub tone_key: Yaml,
    pub year_key: Yaml,

    pub timeline: Timeline<'a>,
    pub races: IndexMap<String, Race>,
    pub regions: IndexMap<String, Region>,
    pub events: IndexMap<String, Event>, 
    pub features: IndexMap<String, GeoFeature>,
}

impl<'a> Caerlun<'a> {
    pub fn new() -> Caerlun<'a> {
        let mut caerlun = Caerlun {
            id_key: Yaml::from_str("id"),
            name_key: Yaml::from_str("name"),
            plural_key: Yaml::from_str("plural"),
            alias_key: Yaml::from_str("alias"),
            parent_key: Yaml::from_str("parent"),
            race_key: Yaml::from_str("race"),
            tone_key: Yaml::from_str("tone"),
            year_key: Yaml::from_str("year"),

            timeline: Timeline::new(),
            races: IndexMap::new(),
            regions: IndexMap::new(),
            events: IndexMap::new(),
            features: IndexMap::new(),
        };

        for p in Asset::iter() {
            if p.ends_with(".yaml") {
                let o = Asset::get(&p);
                match o {
                    Some(cow) => {
                        match std::str::from_utf8(&cow) {
                            Ok(s) => caerlun.build_type(s),
                            _ => (),
                        }
                    },
                    None => (),
                }
            }
        }
        
        caerlun
    }

    fn build_type(&mut self, s: &str) {
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        match doc {
            Yaml::Hash(h) => {
                match &h.get(&Yaml::from_str("regions")) {
                    Some(entry) => {
                        match entry {
                            Yaml::Array(arr) => {
                                for a in arr { self.append_region(&a); }
                            },
                            _ => (),
                        }
                    },
                    None => (),
                }

                match &h.get(&Yaml::from_str("races")) {
                    Some(entry) => {
                        match entry {
                            Yaml::Array(arr) => {
                                for a in arr { self.append_race(&a); }
                            },
                            _ => (),
                        }
                    },
                    None => (),
                }

                match &h.get(&Yaml::from_str("eras")) {
                    Some(entry) => {
                        match entry {
                            Yaml::Array(arr) => {
                                for a in arr { self.append_era(&a); }
                            },
                            _ => (),
                        }
                    },
                    None => (),
                }

                match &h.get(&Yaml::from_str("events")) {
                    Some(entry) => {
                        match entry {
                            Yaml::Array(arr) => {
                                for a in arr { self.append_event(&a); }
                            },
                            _ => (),
                        }
                    },
                    None => (),
                }
            },
            _ => (),
        }

    }

    fn optional_string(&self, opt: Option<&Yaml>) -> Option<String> {
        match opt {
            Some(yaml) => {
                match yaml {
                    Yaml::String(s) => Some(s.to_string()),
                    _ => None,
                }
            },
            None => None,
        }
    }

    fn strings(&self, opt: Option<&Yaml>) -> Vec<String> {
        match opt {
            Some(yaml) => {
                match yaml {
                    Yaml::Array(arr) => arr.iter().map(|s| s.as_str().unwrap().to_string()).collect(),
                    _ => Vec::new()
                }        
            },
            None => Vec::new()
        }
    }

    fn build_aliases(&self, opt: Option<&Yaml>) -> Vec<Alias> {
        match opt {
            Some(yaml) => {
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
            },
            None => Vec::new()
        }
    }

    fn build_alias(&self, yaml: &Yaml) -> Option<Alias> {
        match yaml {
            Yaml::Hash(h) => {
                let tone = match h.get(&self.tone_key) {
                    Some(s) => Tone::from_str(s.as_str().unwrap()).unwrap(),
                    None => Tone::Neutral,
                };
                let alias = Alias{
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    tone: tone,
                    races: self.strings(h.get(&self.race_key)),
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
                    plural: self.optional_string(h.get(&self.plural_key)),
                    alias: self.build_aliases(h.get(&self.alias_key)),
                };
                self.races.insert(id, r);
            },
            _ => panic!("Expected to build race instance from hash"),
        }
    }

    pub fn append_geo(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let id = h[&self.id_key].as_str().unwrap().to_string();
                let g = GeoFeature{
                    id: id.clone(),
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                };
                self.features.insert(id, g);
            },
            _ => panic!("Expected to build a geo instance from hash"),
        }
    }

    pub fn append_region(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let id = h[&self.id_key].as_str().unwrap().to_string();
                let opt_parent = self.optional_string(h.get(&self.parent_key));
                let r = Region{
                    id: id.clone(),
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    plural: self.optional_string(h.get(&self.plural_key)),
                    alias: self.build_aliases(h.get(&self.alias_key)),
                    parent: opt_parent.clone(),
                    children: Vec::new(),
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

    pub fn append_era(&mut self, yaml: &Yaml) {


    }

    pub fn append_event(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let id = h[&self.id_key].as_str().unwrap().to_string();
                let opt_parent = self.optional_string(h.get(&self.parent_key));
                let e = Event{
                    id: id.clone(),
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    alias: self.build_aliases(h.get(&self.alias_key)),
                    range: TimeRange::new(h[&self.year_key].as_str().unwrap()),
                    races: self.strings(h.get(&self.race_key)),
                    parent: opt_parent.clone(),
                    children: Vec::new(),
                };
                self.events.insert(id.clone(), e);
                match opt_parent {
                    Some(parent_id) => {
                        if let Some(parent) = self.events.get_mut(&parent_id) {
                            parent.children.push(id.clone());
                        }
                    },
                    None => (),
                }
            },
            _ => panic!("Expected to build an event instance from hash"),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Race {
    pub id: String,
    pub name: String,
    pub plural: Option<String>,
    pub alias: Vec<Alias>,
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

#[derive(Debug)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub plural: Option<String>,
    pub alias: Vec<Alias>,
    pub parent: Option<String>,
    pub children: Vec<String>,
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Alias {
    name: String,
    tone: Tone,
    races: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug)]
pub struct Era {
    pub id: String,
    pub name: String,
    range: TimeRange,
    pub races: Vec<String>
}

impl Era {
    fn new(id: &str, name: &str) -> Era {
        Era { id: id.to_string(), name: name.to_string(), range: TimeRange::new("1980 to 1990"), races: Vec::new() }
    }
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
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for Era {}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
    range: TimeRange,
    pub alias: Vec<Alias>,
    pub races: Vec<String>,
    pub parent: Option<String>,
    pub children: Vec<String>,
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
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
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
