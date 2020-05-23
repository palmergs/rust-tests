extern crate regex;
use regex::Regex;

use yaml_rust::Yaml;

use super::{ Caerlun, Region, Race, Alias};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
pub struct Event {
    pub key: String,
    pub name: String,
    pub range: Range<i64>,
    pub alias: Vec<Alias>,
    pub races: Vec<String>,
    pub regions: Vec<String>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

impl Event {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref EVENT_KEY: Yaml = Yaml::from_str("race");
        }
        &EVENT_KEY
    }

    pub fn year_key() -> &'static Yaml {
        lazy_static! {
            static ref YEAR_KEY: Yaml = Yaml::from_str("year");
        }
        &YEAR_KEY
    }

    pub fn build(yaml: &Yaml) -> Event {
        match yaml {
            Yaml::Hash(h) => {
                let key = Caerlun::opt_string(h.get(Caerlun::id_key())).expect("missing id key");
                let name = Caerlun::opt_string(h.get(Caerlun::name_key())).expect("missing name key");
                let year = Caerlun::opt_string(h.get(Event::year_key())).expect("missing year key");
                Event{
                    key: key,
                    parent: Caerlun::opt_string(h.get(Caerlun::parent_key())),
                    name: name,
                    range: Event::parse_years(&year),
                    alias: Alias::build(h.get(Alias::key())),
                    races: Caerlun::strings(h.get(Race::key())),
                    regions: Caerlun::strings(h.get(Region::key())),
                    children: Vec::new(),
                }
            },
            _ => panic!("Expected a hash when building an event"),
        }
    }

    pub fn parse_years(time: &str) -> Range<i64> {
        lazy_static! {
            static ref RANGE: Regex = Regex::new(r"\s*(to|before|until|after)\s*").unwrap();
            static ref NUMBER: Regex = Regex::new(r"\s*([-]?[0-9]+)\s*").unwrap();
        }
    
        match RANGE.captures(time) {
            Some(capture) => {
                let nums: Vec<&str> = NUMBER
                    .find_iter(time)
                    .map(|mat| mat.as_str().trim())
                    .collect();
                let one: i64 = nums[0].parse().unwrap();
                match capture.get(1).unwrap().as_str() {
                    "to" => {
                        if nums.len() > 1 {
                            let two: i64 = nums[1].parse().unwrap();
                            one..two
                        } else {
                            one..i64::max_value()
                        }
                    }
                    "before" | "until" => i64::min_value()..one,
                    "after" => one..i64::max_value(),
                    _ => 0..0,
                }
            }
            None => {
                let year = time.to_string();
                let year = year.trim();
                let year: i64 = year.parse().unwrap();
                year..year
            }
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
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Event {}
