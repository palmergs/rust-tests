#[macro_use] extern crate lazy_static;
extern crate regex;
use rand::seq::SliceRandom;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::hash_map::HashMap;

fn get_key(text: &str) -> Result<&str, ()> {
    lazy_static! {
        static ref KEY: Regex = Regex::new(r"^[:]([a-zA-Z0-9_-]+)").unwrap();
    }
    match KEY.captures(text) {
        Some(capture) => Ok(capture.get(1).unwrap().as_str()),
        None => Err(())
    }
}

fn get_header(text: &str) -> Result<&str, ()> {
    lazy_static! {
        static ref SECTION_HEAD: Regex = Regex::new(r"^\[([a-z0-9_-]+)\]$").unwrap();
    }
    match SECTION_HEAD.captures(text) {
        Some(capture) => Ok(capture.get(1).unwrap().as_str()),
        None => Err(())
    }
}

#[derive(Debug)]
pub enum Fragment {
    Constant(String),
    Ident(String),
    Series(Vec<Fragment>),
}

impl Fragment {
    fn new(value: &str) -> Fragment {
       let key = get_key(value);
       match key {
            Ok(key) => Fragment::Ident(key.to_string()),
            Err(_) => Fragment::Constant(value.to_string()),
       }
    }

    fn name(&self, hash: &HashMap<String, FragmentList>) -> String {
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
}

#[derive(Debug)]
pub struct FragmentList {
    ident: String,
    fragments: Vec<Fragment>
}

impl FragmentList {
    fn new(ident: &str) -> FragmentList {
        FragmentList {
            ident: ident.to_string(),
            fragments: Vec::new()
        }
    }

    fn name(&self, hash: &HashMap<String, FragmentList>) -> String {
       let mut rng = rand::thread_rng();
       match self.fragments.choose(&mut rng) {
            Some(frag) => frag.name(&hash),
            None => format!("[{}]", self.ident).to_string(),
       }
    }
}


pub fn parse_into_groups(contents: &str) -> HashMap<String, FragmentList> {
    let mut hash = HashMap::new();
    let mut curr = "".to_string();
    for line in contents.lines() {
        if line != "" {
            let head = get_header(line);
            if head.is_ok() {
                let name = head.unwrap();
                curr = name.to_string();
                hash.insert(name.to_string(), FragmentList::new(name));
            } else {
                match hash.get_mut(&curr) {
                    Some(frag) => {
                        let vec: Vec<&str> = line.split('+').collect();
                        if vec.len() == 1 {
                            let f = Fragment::new(vec.first().unwrap());
                            frag.fragments.push(f);
                        } else {
                            let fs = vec.iter().map(|&x| Fragment::new(x)).collect::<Vec<_>>();
                            frag.fragments.push(Fragment::Series(fs));
                        }
                    },
                    None => (),
                }
            }
        }
    }
    hash
}

pub fn name(hash: &HashMap<String, FragmentList>, key: &str) -> String {
    let mut rng = rand::thread_rng();
    match &hash.get(key) {
        Some(fragment_list) => {
            match fragment_list.fragments.choose(&mut rng) {
                Some(fragment) => {
                    fragment.name(&hash)
                },
                None => "unknown".to_string(),
            }
        },
        None => "unknown".to_string(),
    }
}
