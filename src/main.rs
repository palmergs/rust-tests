#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate clap;
use std::fs;
use std::fmt;
use std::error::Error;
use clap::{App, Arg};
use rand::prelude::*;
use rand::seq::SliceRandom;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::hash_map::HashMap;

fn is_integer(text: &str) -> bool {
    lazy_static! {
        static ref INT: Regex = Regex::new(r"^\d+$").unwrap();
    }
    INT.is_match(text)
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
enum Fragment {
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
struct FragmentList {
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


fn parse_into_groups(contents: &str) -> HashMap<String, FragmentList> {
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

fn name(hash: &HashMap<String, FragmentList>, key: &str) -> String {
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

fn main() {
    let matches = App::new("randomlines")
        .version(include_str!("version"))
        .about("Reads random lines from a file.!")
        .author("Galen P.")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("File to load the name strings")
            .takes_value(true))
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .value_name("KEY")
            .help("Key of the strings to generate")
            .takes_value(true))
        .arg(Arg::with_name("count")
            .short("c")
            .long("count")
            .value_name("COUNT")
            .help("Number of strings to generate")
            .takes_value(true))
       .get_matches();

    let file_name = matches.value_of("file")
        .unwrap_or("names.txt");
    let contents = fs::read_to_string(file_name).expect("unable to read name file");
    let groups = parse_into_groups(&contents);

    let key = matches.value_of("key").unwrap_or("unknown");

    let count_str = matches.value_of("count").unwrap_or("100");
    let count = count_str.parse::<i32>().unwrap();
    for n in 0..count {
        println!("{}. = {}", n, name(&groups, key));
    }
}
