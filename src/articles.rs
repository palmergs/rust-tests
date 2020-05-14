extern crate regex;
// use regex::Regex;

use yaml_rust::{ Yaml, YamlLoader };
// use sorted_vec::SortedVec;
use indexmap::IndexMap;
// use nested_intervals::IntervalSet;

//use std::i32::{ MIN, MAX };
use std::collections::HashMap;
use std::str::FromStr;

use super::{ Race, Region, Event, Geo, Tone, Alias };

// const YEAR_OFFSET: usize = 10000;

#[derive(Debug)]
pub struct Caerlun {
    pub id_key: Yaml,
    pub name_key: Yaml,
    pub abbr_key: Yaml,
    pub plural_key: Yaml,
    pub alias_key: Yaml,
    pub parent_key: Yaml,
    pub race_key: Yaml,
    pub tone_key: Yaml,
    pub year_key: Yaml,

    count: usize,
    ids_to_keys: HashMap<usize, String>,
    keys_to_ids: HashMap<String, usize>,

    pub races: IndexMap<usize, Race>,
    pub regions: IndexMap<usize, Region>,
    pub events: IndexMap<usize, Event>, 
    pub features: IndexMap<usize, Geo>,
}

impl Caerlun {
    pub fn new() -> Caerlun {
        let caerlun = Caerlun {
            id_key: Yaml::from_str("id"),
            name_key: Yaml::from_str("name"),
            abbr_key: Yaml::from_str("abbr"),
            plural_key: Yaml::from_str("plural"),
            alias_key: Yaml::from_str("alias"),
            parent_key: Yaml::from_str("parent"),
            race_key: Yaml::from_str("race"),
            tone_key: Yaml::from_str("tone"),
            year_key: Yaml::from_str("year"),

            count: 0,
            ids_to_keys: HashMap::new(),
            keys_to_ids: HashMap::new(),

            races: IndexMap::new(),
            regions: IndexMap::new(),
            events: IndexMap::new(),
            features: IndexMap::new(),
        };

        caerlun
    }

    // pub fn read_assets(&mut self, 

    pub fn region_by_id(&self, id: usize) -> Option<&Region> {
        self.regions.get::<usize>(&id)
    }

    pub fn region(&self, key: &str) -> Option<&Region> {
        if let Some(id) = self.keys_to_ids.get::<str>(key) {
            return self.regions.get::<usize>(&id)
        }
        None
    }

    pub fn race_by_id(&self, id: usize) -> Option<&Race> {
        self.races.get::<usize>(&id)
    }

    pub fn race(&self, key: &str) -> Option<&Race> {
        if let Some(id) = self.keys_to_ids.get::<str>(key) {
            return self.races.get::<usize>(&id);
        }
        None
    }

    fn register(&mut self, key: &str) -> usize {
        let id = self.count;
        self.ids_to_keys.insert(id, key.to_string());
        self.keys_to_ids.insert(key.to_string(), id);
        self.count = self.count + 1;
        id
    }

    pub fn build_type(&mut self, s: &str) {
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

    fn optional_id(&self, opt: Option<&Yaml>) -> Option<usize> {
        match opt {
            Some(yaml) => {
                match yaml {
                    Yaml::String(s) => {
                        if let Some(n) = self.keys_to_ids.get::<str>(&s) {
                            Some(*n)
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            },
            None => None,
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
            _ => None,
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

    fn ids(&self, opt: Option<&Yaml>) -> Vec<usize> {
        let keys = self.strings(opt);
        let mut vec: Vec<usize> = Vec::new();
        for k in keys {
            if let Some(n) = self.keys_to_ids.get(&k) { vec.push(*n); }
        }
        vec
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
                    races: self.ids(h.get(&self.race_key)),
                };

                Some(alias)
            },
            _ => None
        }
    }    

    pub fn append_race(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let mut r = Race::new(key, name);
                r.plural = self.optional_string(h.get(&self.plural_key));
                r.alias = self.build_aliases(h.get(&self.alias_key));

                let id = self.register(key);
                self.races.insert(id, r);
            },
            _ => panic!("Expected to build race instance from hash"),
        }
    }

    pub fn append_geo(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let g = Geo::new(key, name);

                let id = self.register(key);
                self.features.insert(id, g);
            },
            _ => panic!("Expected to build a geo instance from hash"),
        }
    }

    pub fn append_region(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let parent_id = self.optional_id(h.get(&self.parent_key));
                let mut r = Region::new(key, name);
                r.plural = self.optional_string(h.get(&self.plural_key));
                r.alias = self.build_aliases(h.get(&self.alias_key));
                r.parent = parent_id;

                let id = self.register(key);
                self.regions.insert(id, r);

                match parent_id {
                    Some(n) => {
                        if let Some(parent) = self.regions.get_mut::<usize>(&n) {
                            parent.children.push(id);
                        }
                    },
                    None => (),
                }
            },
            _ => panic!("Expected to build a region instance from hash"),
        }
    }

    pub fn append_event(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let parent_id = self.optional_id(h.get(&self.parent_key));
                let mut e = Event::new(key, name);
                e.alias = self.build_aliases(h.get(&self.alias_key));
                e.range = 0..10;
                e.races = self.ids(h.get(&self.race_key));
                e.parent = parent_id;
                
                let id = self.register(key);
                self.events.insert(id, e);

                match parent_id {
                    Some(n) => {
                        if let Some(parent) = self.events.get_mut::<usize>(&(n as usize)) {
                            parent.children.push(id);
                        }
                    },
                    None => (),
                }
            },
            _ => panic!("Expected to build an event instance from hash"),
        }
    }
}

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
// struct TimeRange {
//     start: Option<i32>,
//     end: Option<i32>,
// }

// impl TimeRange {

//     // Can be built from `### to ###`, `before ###`, `after ###` or `until ###`
//     pub fn new(time: &str) -> TimeRange {
//         lazy_static! {
//             static ref RANGE: Regex = Regex::new(r"\s*(to|before|until|after)\s*").unwrap();
//             static ref NUMBER: Regex = Regex::new(r"\s*([-]?[0-9]+)\s*").unwrap();
//         }
//         match RANGE.captures(time) {
//             Some(capture) => {
//                 let nums: Vec<&str> = NUMBER.find_iter(time).map(|mat| mat.as_str().trim()).collect();
//                 let one: i32 = nums[0].parse().unwrap();
//                 match capture.get(1).unwrap().as_str() {
//                     "to" => {
//                         let two: i32 = nums[1].parse().unwrap();
//                         TimeRange{ start: Some(std::cmp::min(one, two)), end: Some(std::cmp::max(one, two)) }
//                     },
//                     "before" | "until" => TimeRange{ start: None, end: Some(one) },
//                     "after" => TimeRange{ start: Some(one), end: None },
//                     _ => panic!("Unable to parse time range with {}", time),
//                 }
//             },
//             None => {
//                 let year = time.to_string();
//                 let year = year.trim();
//                 let year: i32 = year.parse().unwrap();
//                 TimeRange{ start: Some(year), end: Some(year) }
//             }
//         }
//     }
// }





// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn time_range_from_string() {
//         let range = TimeRange::new("1234");
//         assert_eq!(1234 as i32, range.start.unwrap());
//         assert_eq!(1234 as i32, range.end.unwrap());

//         let range = TimeRange::new(" -432  ");
//         assert_eq!(-432 as i32, range.start.unwrap());
//         assert_eq!(-432 as i32, range.end.unwrap());

//         let range = TimeRange::new("-100 to 200");
//         assert_eq!(-100 as i32, range.start.unwrap());
//         assert_eq!(200 as i32, range.end.unwrap());

//         let range = TimeRange::new("after 1000");
//         assert_eq!(1000 as i32, range.start.unwrap());
//         assert_eq!(None, range.end);

//         let range = TimeRange::new("before 888");
//         assert_eq!(None, range.start);
//         assert_eq!(888, range.end.unwrap());
//     }
// }
