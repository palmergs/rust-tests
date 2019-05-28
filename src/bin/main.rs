extern crate clap;
use rust_names::NameBuilder;
use std::fs;
use clap::{App, Arg};

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
    let contents = fs::read_to_string(file_name)
        .expect("unable to read name file");
    let mut builder = NameBuilder::new();
    builder.parse(&contents);

    println!("NameBuilder={:?}", builder);

    let key = matches.value_of("key").unwrap_or("dwarf");

    let count_str = matches.value_of("count").unwrap_or("100");
    let count = count_str.parse::<i32>().unwrap();
    for _ in 0..count {
        println!("{}", builder.name(key));
    }
}
