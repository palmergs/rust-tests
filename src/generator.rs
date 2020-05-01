extern crate regex;
use regex::Regex;

use std::collections::hash_map::HashMap;
use rand::seq::SliceRandom;

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
            Fragment::Ident(val) => {
                match hash.get(val) {
                    Some(frag) => frag.name(hash),
                    None => "".to_string(),
                }
            },
            Fragment::Series(vec) => {
                let strings: Vec<String> = vec.iter().map(|f| f.name(hash)).collect();
                strings.join("")
            },
        }
    }

    pub fn get_key(text: &str) -> Result<&str, ()> {
        lazy_static! {
            static ref KEY: Regex = Regex::new(r"^[:]([a-zA-Z0-9_-]+)").unwrap();
        }
        match KEY.captures(text) {
            Some(capture) => Ok(capture.get(1).unwrap().as_str()),
            None => Err(())
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
        let anon =  ident.starts_with("_");
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

    pub fn choose(&self) -> Option<&Fragment> {
        let mut rng = rand::thread_rng();
        self.fragments.choose(&mut rng)
    }

    pub fn get_header(text: &str) -> Result<&str, ()> {
        lazy_static! {
            static ref SECTION_HEAD: Regex = Regex::new(r"^\[([a-z0-9_-]+)\]$").unwrap();
        }
        match SECTION_HEAD.captures(text) {
            Some(capture) => Ok(capture.get(1).unwrap().as_str()),
            None => Err(())
        }
    }
}
