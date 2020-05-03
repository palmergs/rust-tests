extern crate clap;
use clap::{App, Arg};

use rust_names::{ NameBuilder, CharacterBuilder };
use std::io;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let matches = App::new("randomlines")
        .version(VERSION)
        .about("Randomly generate strings or structured objects from a file.")
        .author("Galen P.")
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .value_name("KEY")
            .help("Randomly generate strings using the given key")
            .takes_value(true))
        .arg(Arg::with_name("character")
            .short("c")
            .long("character")
            .takes_value(true)
            .help("If set, generate a randomly created character"))
        .arg(Arg::with_name("count")
            .short("n")
            .long("count")
            .value_name("COUNT")
            .help("Number of strings to generate")
            .takes_value(true))
       .get_matches();

    let count = matches.value_of("count").unwrap_or("1");
    let count = count.parse::<i32>().unwrap();

    if matches.is_present("character") {
        let builder = CharacterBuilder::new();
        for _ in 0..count {
            builder.build(
                matches.value_of("key"),
                matches.value_of("race"),
                matches.value_of("region"),
                matches.value_of("dob"))
        }
    } else if let Some(key) = matches.value_of("key") {
        let builder = NameBuilder::new();
        for _ in 0..count {
            println!("{}", builder.name(key));        
        }
    } else {
        let builder = NameBuilder::new();
        for s in builder.keys().iter() {
            println!("{}", s);
        }
    }

    Ok(())
}
