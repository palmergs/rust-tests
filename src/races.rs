extern crate regex;
use regex::Regex;

use super::{Alias, Atts, Stats};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
pub struct Race {
    pub key: String,
    pub name: String,
    pub plural: Option<String>,
    pub alias: Vec<Alias>,
    pub regions: Vec<String>,
    pub height: Range<i64>,
    pub weight: Range<i64>,
    pub lifespan: Vec<Range<i64>>,
    pub mname: String,
    pub fname: String,
    pub lname: Option<String>,
    pub stats: Stats,
    pub atts: Atts,
}

impl Race {
    pub fn new(
        key: &str, 
        name: &str, 
        height: &str, 
        weight: &str, 
        lifespan: &str, 
        mname: &str, 
        fname: &str, 
        lname: Option<&str>) -> Race {

        Race {
            key: key.to_string(),
            name: name.to_string(),
            plural: None,
            alias: Vec::new(),
            regions: Vec::new(),
            height: Race::parse_height(height),
            weight: Race::parse_weight(weight),
            lifespan: Race::parse_lifespan(lifespan),
            mname: mname.to_string(),
            fname: fname.to_string(),
            lname: match lname {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            stats: Race::parse_stats(),
            atts: Race::parse_atts(),
        }
    }

    pub fn pc() -> Vec<&'static str> {
        vec![
            "human", "elf", "dwarf", "rulligg", "feletaur", "centaur", "urunai", "gobru", "urg",
        ]
    }

    fn parse_stats() -> Stats {
        Stats {
            bdy: 10,
            foc: 10,
        }
    }

    fn parse_atts() -> Atts {
        Atts {
            st: 0,
            en: 0,
            dx: 0,
            hc: 0,
            aw: 0,
            it: 0,
            wi: 0,
            ch: 0,
        }
    }

    fn parse_height(height: &str) -> Range<i64> {
        lazy_static! {
            static ref HEIGHT: Regex = Regex::new("\\s*(\\d+)'(\\d+)\"\\s*-\\s*(\\d+)'(\\d+)\"\\s*").unwrap();
        }

        match HEIGHT.captures(height) {
            Some(captures) => {
                if captures.len() < 5 {
                    panic!("Expected four numbers from height: value={} captures={:?}", height, captures);
                }
                let min_ft: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let min_in: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                let max_ft: i64 = captures.get(3).unwrap().as_str().parse().unwrap();
                let max_in: i64 = captures.get(4).unwrap().as_str().parse().unwrap();
                (min_ft * 12 + min_in)..(max_ft * 12 + max_in)
            },
            None => panic!("Could not parse height: value={}", height),
        }
    }

    fn parse_weight(weight: &str) -> Range<i64> {
        lazy_static! {
            static ref WEIGHT: Regex = Regex::new("\\s*(\\d+)\\s*\\-\\s*(\\d+)\\s*").unwrap();
        }   

         match WEIGHT.captures(weight) {
            Some(captures) => {
                if captures.len() < 3 {
                    panic!("Expected two numbers from weight: value={} captures={:?}", weight, captures);
                }
                let min: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let max: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                min..max
            },
            None => panic!("Could not parse weight: value={}", weight),
        }     
    }

    fn parse_lifespan(lifespan: &str) -> Vec<Range<i64>> {
        lazy_static! {
            static ref NUM_RANGE: Regex = Regex::new("\\s*(\\d+)\\s*\\-\\s*(\\d+)\\s*").unwrap();
        } 

        let mut ranges: Vec<Range<i64>> = Vec::new();
        for s in lifespan.split(",") {
            match NUM_RANGE.captures(s) {
                Some(captures) => {
                    if captures.len() < 3 {
                        panic!("Expected two numbers from lifespan entry: value={} captures={:?}", s, captures);
                    }
                    let min: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let max: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                    ranges.push(min..max);
                },
                None => panic!("Could not parse a number pair: value={}", s),
            }
        }

        if ranges.len() != 4 {
            panic!("Expected exactly 4 number pairs: ranges={:?}", ranges);
        }
        ranges
    }
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Race {}

impl Ord for Race {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Race {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
