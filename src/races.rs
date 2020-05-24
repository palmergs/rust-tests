extern crate regex;
use regex::Regex;

use rand::Rng;

use yaml_rust::Yaml;

use super::{Alias, Value, Caerlun, Region, POINTS, ATTRIBUTES};
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
    pub points:[Value; POINTS],
    pub attributes:[Value; ATTRIBUTES],
}

impl Race {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref RACE_KEY: Yaml = Yaml::from_str("race");
        }
        &RACE_KEY
    }

    pub fn male_name_key() -> &'static Yaml {
        lazy_static! {
            static ref MNAME_KEY: Yaml = Yaml::from_str("mname");
        }
        &MNAME_KEY
    }

    pub fn female_name_key() -> &'static Yaml {
        lazy_static! {
            static ref FNAME_KEY: Yaml = Yaml::from_str("fname");
        }
        &FNAME_KEY
    }

    pub fn family_name_key() -> &'static Yaml {
        lazy_static! {
            static ref LNAME_KEY: Yaml = Yaml::from_str("lname");
        }
        &LNAME_KEY
    }

    pub fn height_key() -> &'static Yaml {
        lazy_static! {
            static ref RACE_KEY: Yaml = Yaml::from_str("height");
        }
        &RACE_KEY
    }

    pub fn weight_key() -> &'static Yaml {
        lazy_static! {
            static ref RACE_KEY: Yaml = Yaml::from_str("weight");
        }
        &RACE_KEY
    }

    pub fn lifespan_key() -> &'static Yaml {
        lazy_static! {
            static ref RACE_KEY: Yaml = Yaml::from_str("lifespan");
        }
        &RACE_KEY
    }

    pub fn player_races() -> Vec<&'static str> {
        vec![
            "human", "elf", "dwarf", "rulligg", "feletaur", "centaur", "urunai", "gobru", "urg",
        ]
    }

    pub fn random_player_race() -> &'static str {
        let mut rng = rand::thread_rng();
        Race::player_races()[rng.gen_range(0, Race::player_races().len())]
    }

    pub fn build(yaml: &Yaml) -> Race {
        match yaml {
            Yaml::Hash(h) => {
                let key = Caerlun::opt_string(h.get(Caerlun::id_key())).expect("missing id key");
                let name =
                    Caerlun::opt_string(h.get(Caerlun::name_key())).expect("missing name key");
                let mname = Caerlun::opt_string(h.get(Race::male_name_key()))
                    .expect("missing male name key");
                let fname = Caerlun::opt_string(h.get(Race::female_name_key()))
                    .expect("missing female name key");
                let lname = Caerlun::opt_string(h.get(Race::family_name_key()));
                let height =
                    Caerlun::opt_string(h.get(Race::height_key())).expect("Expected height key");
                let weight =
                    Caerlun::opt_string(h.get(Race::weight_key())).expect("Expected weight key");
                let lifespan = Caerlun::opt_string(h.get(Race::lifespan_key()))
                    .expect("Expected lifespan key");
                Race {
                    key: key,
                    name: name,
                    plural: Caerlun::opt_string(h.get(Caerlun::plural_key())),
                    alias: Alias::build(h.get(Alias::key())),
                    height: Race::parse_height(&height),
                    weight: Race::parse_weight(&weight),
                    lifespan: Race::parse_lifespan(&lifespan),
                    mname: mname,
                    fname: fname,
                    lname: match lname {
                        Some(s) => Some(s),
                        None => None,
                    },
                    regions: Caerlun::strings(h.get(Region::key())),
                    points: Value::build_points(&h.get(Value::points_key()).expect("missing points key")),
                    attributes: Value::build_attributes(&h.get(Value::attributes_key()).expect("missing attributes key")),
                }
            }
            _ => panic!("expected a hash to build a race intance"),
        }
    }

    fn parse_height(height: &str) -> Range<i64> {
        lazy_static! {
            static ref HEIGHT: Regex =
                Regex::new("\\s*(\\d+)'(\\d+)\"\\s*-\\s*(\\d+)'(\\d+)\"\\s*").unwrap();
        }

        match HEIGHT.captures(height) {
            Some(captures) => {
                if captures.len() < 5 {
                    panic!(
                        "Expected four numbers from height: value={} captures={:?}",
                        height, captures
                    );
                }
                let min_ft: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let min_in: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                let max_ft: i64 = captures.get(3).unwrap().as_str().parse().unwrap();
                let max_in: i64 = captures.get(4).unwrap().as_str().parse().unwrap();
                (min_ft * 12 + min_in)..(max_ft * 12 + max_in)
            }
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
                    panic!(
                        "Expected two numbers from weight: value={} captures={:?}",
                        weight, captures
                    );
                }
                let min: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let max: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                min..max
            }
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
                        panic!(
                            "Expected two numbers from lifespan entry: value={} captures={:?}",
                            s, captures
                        );
                    }
                    let min: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let max: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
                    ranges.push(min..max);
                }
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
