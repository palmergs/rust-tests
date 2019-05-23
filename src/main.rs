#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate clap;
use std::fs;
use std::error::Error;
use clap::{App, Arg};
use rand::prelude::*;
use rand::seq::SliceRandom;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::hash_map::HashMap;

fn is_integer(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d+$").unwrap();
    }
    RE.is_match(text)
}

fn get_header(text: &str) -> &str {
    lazy_static! {
        static ref SECTION_HEAD: Regex = Regex::new(r"^\[([a-z0-9\s]+)\]$").unwrap();
    }
    match SECTION_HEAD.find(text) {
        Some(capture) => capture.as_str(),
        None => ""
    }
}

#[derive(Debug)]
struct FragmentList {
    name: String,
    fragments: Vec<String>
}

impl FragmentList {
    fn new(name: &str) -> FragmentList {
        FragmentList {
            name: name.to_string(),
            fragments: Vec::new()
        }
    }
}

fn parse_into_groups(contents: &str) -> HashMap<String, FragmentList> {
    let mut hash = HashMap::new();
    let mut curr = "".to_string();
    for line in contents.lines() {
        if line != "" {
            let name = get_header(line);
            if name != "" {
                curr = name.to_string();
                hash.insert(name.to_string(), FragmentList::new(name));
            } else {
                match hash.get_mut(&curr) {
                    Some(list) => list.fragments.push(line.to_string()),
                    None => (),
                }
            }
        }
    }
    hash
}

fn name(hash: &HashMap<String, FragmentList>) -> String {
    let mut rng = rand::thread_rng();
    let group0 = &hash.get("[dfirst]").unwrap().fragments;
    let group1 = &hash.get("[dlast]").unwrap().fragments;
    format!("{}{}", 
        group0.choose(&mut rng).unwrap(), 
        group1.choose(&mut rng).unwrap())
}

fn main() {
    let matches = App::new("randomlines")
       .version("1.0")
       .about("Reads random lines from a file.!")
       .author("Galen P.")
       .arg(Arg::with_name("file")
           .short("f")
           .long("file")
           .value_name("FILE")
           .help("File to load the name strings")
           .takes_value(true))
       .get_matches();

    println!("VERSION={}", include_str!("version"));

    let file_name = matches.value_of("file")
        .unwrap_or("names.txt");
    let contents = fs::read_to_string(file_name).expect("unable to read name file");
    let groups = parse_into_groups(&contents);


    println!("hash map = {:?}", groups);

    for n in 0..100 {
        println!("{}. = {}", n, name(&groups));
    }

    let n32 = rand::random::<i32>();
    let n64 = rand::random::<i64>();
    println!("A random number: {}", n32);
    println!("A random number: {}", n64);
    println!("Is it a integer? {}", is_integer(&"1234".to_string()));
}
