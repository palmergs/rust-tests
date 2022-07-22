extern crate regex;
use regex::Regex;

use rand::seq::SliceRandom;
use std::collections::hash_map::HashMap;

use super::Asset;

#[derive(Debug)]
pub enum Fragment {
    Constant(String),
    Ident(String),
    Series(Vec<Fragment>),
}

impl Fragment {
    pub fn new(value: &str) -> Fragment {
        let key = Fragment::get_key(value);
        match key {
            Ok(key) => Fragment::Ident(key.to_string()),
            Err(_) => Fragment::Constant(value.to_string()),
        }
    }

    pub fn name(&self, hash: &HashMap<String, FragmentList>) -> String {
        match self {
            Fragment::Constant(val) => val.to_string(),
            Fragment::Ident(val) => match hash.get(val) {
                Some(frag) => frag.name(hash),
                None => "".to_string(),
            },
            Fragment::Series(vec) => {
                let strings: Vec<String> = vec.iter().map(|f| f.name(hash)).collect();
                strings.join("")
            }
        }
    }

    pub fn get_key(text: &str) -> Result<&str, ()> {
        lazy_static! {
            static ref KEY: Regex = Regex::new(r"^[:]([a-zA-Z0-9_-]+)").unwrap();
        }
        match KEY.captures(text) {
            Some(capture) => Ok(capture.get(1).unwrap().as_str()),
            None => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct FragmentList {
    pub ident: String,
    fragments: Vec<Fragment>,
    anonymous: bool,
}

impl FragmentList {
    pub fn new(ident: &str) -> FragmentList {
        let anon = ident.starts_with("_");
        let ident = ident.to_string();
        FragmentList {
            ident: ident,
            fragments: Vec::new(),
            anonymous: anon,
        }
    }

    pub fn name(&self, hash: &HashMap<String, FragmentList>) -> String {
        let mut rng = rand::thread_rng();
        match self.fragments.choose(&mut rng) {
            Some(frag) => frag.name(&hash),
            None => format!("[{}]", self.ident).to_string(),
        }
    }

    pub fn add(&mut self, frag: Fragment) {
        self.fragments.push(frag);
    }

    pub fn choose(&self, hash: &HashMap<String, FragmentList>) -> String {
        let mut rng = rand::thread_rng();
        match self.fragments.choose(&mut rng) {
            Some(fragment) => fragment.name(hash),
            None => "<frag>".to_string(),
        }
    }

    pub fn get_header(text: &str) -> Result<&str, ()> {
        lazy_static! {
            static ref SECTION_HEAD: Regex = Regex::new(r"^\[([a-z0-9_-]+)\]$").unwrap();
        }
        match SECTION_HEAD.captures(text) {
            Some(capture) => Ok(capture.get(1).unwrap().as_str()),
            None => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct NameBuilder {
    hash: HashMap<String, FragmentList>,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        let mut builder = NameBuilder {
            hash: HashMap::new(),
        };
        for p in Asset::iter() {
            if p.ends_with(".txt") {
                let o = Asset::get(&p);
                match o {
                    Some(cow) => match std::str::from_utf8(&cow.data) {
                        Ok(s) => builder.parse(s),
                        _ => (),
                    },
                    None => (),
                };
            }
        }
        return builder;
    }

    fn parse(&mut self, contents: &str) {
        let mut curr_ident = "".to_string();
        for line in contents.lines() {
            if line != "" {
                let head = FragmentList::get_header(line);
                if head.is_ok() {
                    let name = head.unwrap();
                    curr_ident = name.to_string();
                    self.hash.insert(name.to_string(), FragmentList::new(name));
                } else {
                    match self.hash.get_mut(&curr_ident) {
                        Some(frag) => {
                            let vec: Vec<&str> = line.split('+').collect();
                            if vec.len() == 1 {
                                let f = Fragment::new(vec.first().unwrap());
                                frag.add(f);
                            } else {
                                let fs = vec.iter().map(|&x| Fragment::new(x)).collect::<Vec<_>>();
                                frag.add(Fragment::Series(fs));
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }

    // Given a key, return a string generated from that key.
    // Returns <list> if the key was not found
    pub fn name(&self, key: &str) -> String {
        match &self.hash.get(key) {
            Some(fragment_list) => fragment_list.choose(&self.hash),
            None => "<list>".to_string(),
        }
    }

    // Return all non-anonlymous keys included in the builder
    pub fn keys(&self) -> Vec<String> {
        self.hash
            .keys()
            .filter(|s| !s.starts_with("_"))
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    // Return all keys included in the builder
    pub fn all_keys(&self) -> Vec<String> {
        self.hash
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}
