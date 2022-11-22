use indexmap::IndexMap;
use rand::Rng;
use std::cmp::{max, min};
use yaml_rust::{Yaml, YamlLoader};

use super::{Event, Geo, Race, Region};

pub const EARLIEST_YEAR: i64 = -5000;
pub const CURRENT_YEAR: i64 = 1260;

const TIMELINE_WIDTH: i64 = 100;

#[derive(Debug)]
pub struct Caerlun {
    pub races: IndexMap<String, Race>,
    pub regions: IndexMap<String, Region>,
    pub events: IndexMap<String, Event>,
    pub features: IndexMap<String, Geo>,

    pub leaf_regions: Vec<String>,
}

impl Caerlun {
    pub fn id_key() -> &'static Yaml {
        lazy_static! {
            static ref ID_KEY: Yaml = Yaml::from_str("id");
        }
        &ID_KEY
    }

    pub fn name_key() -> &'static Yaml {
        lazy_static! {
            static ref NAME_KEY: Yaml = Yaml::from_str("name");
        }
        &NAME_KEY
    }

    pub fn plural_key() -> &'static Yaml {
        lazy_static! {
            static ref PLURAL_KEY: Yaml = Yaml::from_str("plural");
        }
        &PLURAL_KEY
    }

    pub fn parent_key() -> &'static Yaml {
        lazy_static! {
            static ref PARENT_KEY: Yaml = Yaml::from_str("parent");
        }
        &PARENT_KEY
    }

    pub fn new() -> Caerlun {
        let caerlun = Caerlun {
            races: IndexMap::new(),
            regions: IndexMap::new(),
            events: IndexMap::new(),
            features: IndexMap::new(),

            leaf_regions: Vec::new(),
        };

        caerlun
    }

    pub fn race(&self, key: &str) -> Option<&Race> {
        self.races.get(key)
    }

    pub fn event(&self, key: &str) -> Option<&Event> {
        self.events.get(key)
    }

    pub fn feature(&self, key: &str) -> Option<&Geo> {
        self.features.get(key)
    }

    pub fn region(&self, key: &str) -> Option<&Region> {
        self.regions.get(key)
    }

    pub fn leaf_region(&self, dob: i64, race: Option<&str>) -> &Region {
        let len = self.leaf_regions.len();
        if len == 0 {
            panic!("no regions to select leaf from");
        }

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..len);
        let mut cnt = idx;

        // randomly select a region; if it fails to meet the criteria
        // then get the next one until one that matches the criteria
        // is found; if none is ever found after a complete loop then
        // return the last one
        loop {
            let key = self.leaf_regions.get(cnt).unwrap();
            let region = self.regions.get(key).unwrap();
            if !region.is_water() {
                if region.in_range(dob) {
                    match race {
                        Some(key) => {
                            if region.has_race(key) {
                                return region;
                            }
                        }
                        None => {
                            return region;
                        }
                    }
                }
            }

            cnt = cnt + 1;
            if cnt == idx {
                return region;
            }
            if cnt == len {
                cnt = 0;
            }
        }
    }

    // Display a timeline between two years
    pub fn timeline(&self, start: Option<&str>, end: Option<&str>) {
        let start = match start {
            Some(n) => n.parse::<i64>().expect("could not parse value into year"),
            None => EARLIEST_YEAR,
        };

        let end = match end {
            Some(n) => n.parse::<i64>().expect("could not parse value into year"),
            None => CURRENT_YEAR,
        };

        let total_years: i64 = if start < 0 {
            -1 * start + end
        } else {
            end - start
        };

        let years_per_char: i64 = total_years / TIMELINE_WIDTH;
        let mut events: Vec<&Event> = self.events.values().collect();
        events.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        for e in events {
            if e.range.start > start {
                let event_start = max(start, e.range.start);
                let event_end = min(e.range.end, end);
                let event_years = (event_end - event_start) as i64;
                let event_width = (event_years / years_per_char) as usize;
                let event_offset = ((event_start + (-1 * start)) / years_per_char) as usize;
                println!(
                    "{:>30} {:>5} to {:<5} {:o$}{:*<w$}",
                    e.name,
                    event_start,
                    event_end,
                    " ",
                    "*",
                    o = event_offset,
                    w = event_width
                );
            }
        }
    }

    pub fn print_regions(&self) {
        let regions = self.regions.values();
        for r in regions {
            match r.parent {
                None => {
                    println!("{}", r.name);
                    self.print_recurse_regions(r, 2);
                }
                _ => (),
            }
        }
    }

    fn print_recurse_regions(&self, region: &Region, depth: usize) {
        if depth > 99 {
            println!("Currently looking at region {:?}", region);
            panic!("Depth greater than 5!");
        }
        for id in &region.children {
            let r = &self.regions[id];
            println!("{:depth$}{}", " ", r.name, depth = depth);
            self.print_recurse_regions(&r, depth + 1);
        }
    }

    pub fn build_type(&mut self, s: &str) {
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        match doc {
            Yaml::Hash(h) => {
                match &h.get(&Yaml::from_str("regions")) {
                    Some(entry) => match entry {
                        Yaml::Array(arr) => {
                            for a in arr {
                                self.append_region(&a);
                            }
                        }
                        _ => (),
                    },
                    None => (),
                }

                match &h.get(&Yaml::from_str("races")) {
                    Some(entry) => match entry {
                        Yaml::Array(arr) => {
                            for a in arr {
                                self.append_race(&a);
                            }
                        }
                        _ => (),
                    },
                    None => (),
                }

                match &h.get(&Yaml::from_str("events")) {
                    Some(entry) => match entry {
                        Yaml::Array(arr) => {
                            for a in arr {
                                self.append_event(&a);
                            }
                        }
                        _ => (),
                    },
                    None => (),
                }
            }
            _ => (),
        }
    }

    pub fn find_leaves(&mut self) {
        for (k, region) in &self.regions {
            if region.children.len() == 0 {
                self.leaf_regions.push(k.to_string())
            }
        }
    }

    pub fn append_race(&mut self, yaml: &Yaml) {
        let r = Race::build(yaml);
        self.races.insert(r.key.to_string(), r);
    }

    pub fn append_geo(&mut self, yaml: &Yaml) {
        let g = Geo::build(yaml);
        self.features.insert(g.key.to_string(), g);
    }

    pub fn append_region(&mut self, yaml: &Yaml) {
        let r = Region::build(yaml);
        if let Some(k) = &r.parent {
            if let Some(parent) = self.regions.get_mut(k) {
                parent.children.push(r.key.to_string());
            }
        }
        self.regions.insert(r.key.to_string(), r);
    }

    pub fn append_event(&mut self, yaml: &Yaml) {
        let e = Event::build(yaml);
        if let Some(k) = &e.parent {
            if let Some(parent) = self.events.get_mut(k) {
                parent.children.push(e.key.to_string());
            }
        }
        self.events.insert(e.key.to_string(), e);
    }

    // Build a string from a YAML struct that must exist
    pub fn string(yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::Integer(n) => Some(n.to_string()),
            _ => None,
        }
    }

    pub fn opt_integer(opt: Option<&Yaml>) -> Option<i32> {
        match opt {
            Some(yaml) => match yaml {
                Yaml::String(s) => Some(s.parse::<i32>().expect("Could not parse to integer")),
                Yaml::Integer(n) => Some(*n as i32),
                _ => panic!("could not parse number from field"),
            },
            _ => None,
        }
    }

    // Build a string from a YAML struct that might exist
    pub fn opt_string(opt: Option<&Yaml>) -> Option<String> {
        match opt {
            Some(yaml) => Caerlun::string(yaml),
            _ => None,
        }
    }

    // Turn a YAML list into an array of strings; safely
    // return an empty vector if no list is found.
    pub fn strings(opt: Option<&Yaml>) -> Vec<String> {
        match opt {
            Some(yaml) => match yaml {
                Yaml::Array(arr) => arr
                    .iter()
                    .map(|s| s.as_str().unwrap().to_string())
                    .collect(),
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    }
}
