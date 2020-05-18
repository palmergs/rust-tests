extern crate regex;
use regex::Regex;

use std::cmp::Ordering;
use std::ops::Range;
use super::Alias;

#[derive(Debug)]
pub struct Event {
    pub key: String,
    pub name: String,
    pub range: Range<i64>,
    pub alias: Vec<Alias>,
    pub races: Vec<String>,
    pub parent: Option<String>,
    pub children: Vec<String>,
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

pub fn parse_years(time: &str) -> Range<i64> {
    lazy_static! {
        static ref RANGE: Regex = Regex::new(r"\s*(to|before|until|after)\s*").unwrap();
        static ref NUMBER: Regex = Regex::new(r"\s*([-]?[0-9]+)\s*").unwrap();
    }

    match RANGE.captures(time) {
        Some(capture) => {
            let nums: Vec<&str> = NUMBER.find_iter(time).map(|mat| mat.as_str().trim()).collect();
            let one: i64 = nums[0].parse().unwrap();
            match capture.get(1).unwrap().as_str() {
                "to" => {
                    if nums.len() > 1 {
                        let two: i64 = nums[1].parse().unwrap();
                        one..two
                    } else {
                        one..i64::max_value()
                    }
                },
                "before" | "until" => i64::min_value()..one,
                "after" => one..i64::max_value(),
                _ => 0..0,
            }
        },
        None => {
            let year = time.to_string();
            let year = year.trim();
            let year: i64 = year.parse().unwrap();
            year..year
        }
    }
}
