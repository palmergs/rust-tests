#[macro_use] extern crate lazy_static;
extern crate regex;
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::hash_map::HashMap;


#[derive(Debug)]
pub enum Fragment {
    Constant(String),
    Ident(String),
    Series(Vec<Fragment>),
}

impl Fragment {
    fn new(value: &str) -> Fragment {
       let key = Fragment::get_key(value);
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

    fn get_key(text: &str) -> Result<&str, ()> {
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
    ident: String,
    fragments: Vec<Fragment>,
    anonymous: bool,
}

impl FragmentList {
    fn new(ident: &str) -> FragmentList {
        let anon =  ident.starts_with("_");
        let ident = ident.to_string();
        FragmentList {
            ident: ident,
            fragments: Vec::new(),
            anonymous: anon,
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

#[derive(Debug)]
pub struct NameBuilder {
    hash: HashMap<String, FragmentList>,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        return NameBuilder { hash: HashMap::new() }
    }

    pub fn parse(&mut self, contents: &str) -> Result<(), String> {
        let mut curr_ident = "".to_string();
        for line in contents.lines() {
            if line != "" {
                let head = NameBuilder::get_header(line);
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
        Ok(())
    }

    pub fn name(&self, key: &str) -> String {
        let mut rng = rand::thread_rng();
        match &self.hash.get(key) {
            Some(fragment_list) => {
                match fragment_list.fragments.choose(&mut rng) {
                    Some(fragment) => {
                        fragment.name(&self.hash)
                    },
                    None => "<frag>".to_string(),
                }
            },
            None => "<list>".to_string(),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.hash
            .keys()
            .filter(|s| !s.starts_with("_") )
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn all_keys(&self) ->Vec<String> {
        self.hash
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
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
}
