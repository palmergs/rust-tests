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

fn parse_into_groups(contents: &str) -> Vec<FragmentList> {

    let mut groups: Vec<FragmentList> = Vec::new();
    let mut curr = 0;
    for line in contents.lines() {
        if line != "" {
            let name = get_header(line);
            if name != "" {
                groups.push(FragmentList::new(name));
                curr += 1;
            } else if groups.len() > 0 {
                groups[curr - 1].fragments.push(line.to_string());
            }
        }
    }
    groups
}

fn read_file_into_list(path: &str) -> Vec<&FragmentList> {

    //let contents = fs::read_to_string(file_name)
    //    .expect("unable to read file");
    //let lines: Vec<&str> = contents.split("\n").collect();
    //let mut lists: Vec<&FragmentList> = Vec::new();
    //let mut curr: &mut FragmentList;
    //for line in &lines {
    //    let name = get_header(line);
    //    if name != "" {
    //        println!("{}", name);
    //        curr = &mut FragmentList{ name: name, fragments: Vec::new() };
    //        lists.push(curr)
    //    } else {
    //        curr.fragments.push(name);
    //    }
    //}
    Vec::new()
}

fn name(groups: &Vec<FragmentList>) -> String {
    if groups.len() > 1 {
        let group0 = &groups[0].fragments;
        let group1 = &groups[1].fragments;
        format!("{}{}", 
            group0.choose(&mut rand::thread_rng()).unwrap(), 
            group1.choose(&mut rand::thread_rng()).unwrap())
    } else {
        "".to_string()
    }
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

    let file_name = matches.value_of("file")
        .unwrap_or("names.txt");
    let contents = fs::read_to_string(file_name).expect("unable to read name file");
    let groups = parse_into_groups(&contents);
    for wrapper in &groups {
        println!("What is in the box? {:?}", wrapper);
    }

    for n in 0..100 {
        println!("{}. = {}", n, name(&groups));
    }

    let n32 = rand::random::<i32>();
    let n64 = rand::random::<i64>();
    println!("A random number: {}", n32);
    println!("A random number: {}", n64);
    println!("Is it a integer? {}", is_integer(&"1234".to_string()));
}
