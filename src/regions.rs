use std::cmp::Ordering;
use std::ops::Range;

use yaml_rust::Yaml;

use super::{Caerlun, Race, Alias, Event};

#[derive(Debug)]
pub struct Region {
    pub key: String,
    pub name: String,
    pub plural: Option<String>,
    pub category: Option<String>,
    pub alias: Vec<Alias>,
    pub races: Vec<String>,
    pub range: Option<Range<i64>>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

impl Region {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref REGION_KEY: Yaml = Yaml::from_str("region");
        }
        &REGION_KEY
    }

    pub fn category_key() -> &'static Yaml {
        lazy_static! {
            static ref CATEGORY_KEY: Yaml = Yaml::from_str("category");
        }
        &CATEGORY_KEY
    }

    pub fn build(yaml: &Yaml) -> Region {
        match yaml {
            Yaml::Hash(h) => {
                let key = Caerlun::opt_string(h.get(Caerlun::id_key())).expect("missing id key");
                let name = Caerlun::opt_string(h.get(Caerlun::name_key())).expect("missing name key");
                let cat = Caerlun::opt_string(h.get(Region::category_key()));
                let year = Caerlun::opt_string(h.get(Event::year_key()));
                Region{
                    key: key,
                    parent: Caerlun::opt_string(h.get(Caerlun::parent_key())),
                    name: name,
                    plural: None,
                    alias: Alias::build(h.get(Alias::key())),
                    category: cat,
                    races: Caerlun::strings(h.get(Race::key())),
                    range: match year {
                        Some(s) => Some(Event::parse_years(&s)),
                        None => None,
                    },
                    children: Vec::new(),
                }
            },
            _ => panic!("Expected a hash element when building a region")
        }
    }

    pub fn in_range(&self, year: i64) -> bool {
        match &self.range {
            Some(range) => range.start <= year && range.end >= year,
            None => true,
        }
    }

    pub fn has_race(&self, key: &str) -> bool {
        if self.races.len() == 0 {
            return true;
        }
        for r in &self.races {
            if r == key {
                return true;
            }
        }
        false
    }

    pub fn is_water(&self) -> bool {
        match &self.category {
            Some(c) => c == "body-of-water",
            None => false,
        }
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Region {}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
