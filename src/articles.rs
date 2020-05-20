extern crate regex;
// use regex::Regex;

use yaml_rust::{Yaml, YamlLoader};
// use sorted_vec::SortedVec;
use indexmap::IndexMap;
// use nested_intervals::IntervalSet;

//use std::i32::{ MIN, MAX };
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

use rand::Rng;

use super::{Alias, Event, Geo, Race, Region, Tone};

// const YEAR_OFFSET: usize = 10000;

#[derive(Debug)]
pub struct Caerlun {
    pub id_key: Yaml,
    pub name_key: Yaml,
    pub abbr_key: Yaml,
    pub plural_key: Yaml,
    pub alias_key: Yaml,
    pub parent_key: Yaml,
    pub region_key: Yaml,
    pub race_key: Yaml,
    pub tone_key: Yaml,
    pub year_key: Yaml,
    pub height_key: Yaml,
    pub weight_key: Yaml,
    pub lifespan_key: Yaml,

    pub races: IndexMap<String, Race>,
    pub regions: IndexMap<String, Region>,
    pub events: IndexMap<String, Event>,
    pub features: IndexMap<String, Geo>,

    pub leaf_regions: Vec<String>,
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
            region_key: Yaml::from_str("region"),
            tone_key: Yaml::from_str("tone"),
            year_key: Yaml::from_str("year"),
            height_key: Yaml::from_str("height"),
            weight_key: Yaml::from_str("weight"),
            lifespan_key: Yaml::from_str("lifespan"),

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
        let idx = rng.gen_range(0, len);
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

    pub fn timeline(&self) {
        let start: i64 = -5000;
        let end: i64 = 1260;
        let mut events: Vec<&Event> = self.events.values().collect();
        events.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        for e in events {
            let start = max(start, e.range.start);
            let end = min(e.range.end, end);
            let total = (5000 + 1300) as i64;
            let per = total / 100;
            let offset = ((start + 5000) / per) as usize;
            let width = (((end - start) / per) + 1) as usize;
            println!(
                "{:>30} {:>5} to {:<5} {:o$}{:*<w$}",
                e.name,
                start,
                end,
                " ",
                "*",
                o = offset,
                w = width
            );
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

    fn string(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::Integer(n) => Some(n.to_string()),
            _ => None,
        }
    }

    fn optional_string(&self, opt: Option<&Yaml>) -> Option<String> {
        match opt {
            Some(yaml) => self.string(yaml),
            _ => None,
        }
    }

    fn strings(&self, opt: Option<&Yaml>) -> Vec<String> {
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

    fn build_aliases(&self, opt: Option<&Yaml>) -> Vec<Alias> {
        match opt {
            Some(yaml) => match yaml {
                Yaml::Array(arr) => {
                    let mut vec = Vec::new();
                    for a in arr {
                        match self.build_alias(a) {
                            Some(alias) => vec.push(alias),
                            None => (),
                        }
                    }
                    vec
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    }

    fn build_alias(&self, yaml: &Yaml) -> Option<Alias> {
        match yaml {
            Yaml::Hash(h) => {
                let tone = match h.get(&self.tone_key) {
                    Some(s) => Tone::from_str(s.as_str().unwrap()).unwrap(),
                    None => Tone::Neutral,
                };

                let alias = Alias {
                    name: h[&self.name_key].as_str().unwrap().to_string(),
                    tone: tone,
                    races: self.strings(h.get(&self.race_key)),
                };

                Some(alias)
            }
            _ => None,
        }
    }

    pub fn append_race(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().expect("Expected id key");
                let name = h[&self.name_key].as_str().expect("Expected name key");
                let height = h[&self.height_key].as_str().expect("Expected height key");
                let weight = h[&self.weight_key].as_str().expect("Expected weight key");
                let lifespan = h[&self.lifespan_key].as_str().expect("Expected lifespan key");
                let mut r = Race::new(key, name, height, weight, lifespan);
                r.plural = self.optional_string(h.get(&self.plural_key));
                r.alias = self.build_aliases(h.get(&self.alias_key));

                self.races.insert(key.to_string(), r);
            }
            _ => panic!("Expected to build race instance from hash"),
        }
    }

    pub fn append_geo(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let g = Geo::new(key, name);

                self.features.insert(key.to_string(), g);
            }
            _ => panic!("Expected to build a geo instance from hash"),
        }
    }

    pub fn append_region(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let parent_key = self.optional_string(h.get(&self.parent_key));
                let year = self.optional_string(h.get(&self.year_key));
                let mut r = Region::new(key, name, year.as_deref());
                r.plural = self.optional_string(h.get(&self.plural_key));
                r.alias = self.build_aliases(h.get(&self.alias_key));
                r.races = self.strings(h.get(&self.race_key));

                if let Some(k) = parent_key {
                    r.parent = Some(k.to_string());
                    if let Some(parent) = self.regions.get_mut(&k) {
                        parent.children.push(key.to_string());
                    }
                }

                self.regions.insert(key.to_string(), r);
            }
            _ => panic!("Expected to build a region instance from hash"),
        }
    }

    pub fn append_event(&mut self, yaml: &Yaml) {
        match yaml {
            Yaml::Hash(h) => {
                let key = h[&self.id_key].as_str().unwrap();
                let name = h[&self.name_key].as_str().unwrap();
                let parent_key = self.optional_string(h.get(&self.parent_key));
                let mut e = Event::new(key, name, &self.string(&h[&self.year_key]).unwrap());
                e.alias = self.build_aliases(h.get(&self.alias_key));
                e.races = self.strings(h.get(&self.race_key));
                e.regions = self.strings(h.get(&self.region_key));

                if let Some(k) = parent_key {
                    e.parent = Some(k.to_string());
                    if let Some(parent) = self.events.get_mut(&k) {
                        parent.children.push(key.to_string());
                    }
                }

                self.events.insert(key.to_string(), e);
            }
            _ => panic!("Expected to build an event instance from hash"),
        }
    }
}
