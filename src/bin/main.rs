extern crate clap;
use clap::{App, Arg};

use rust_names::{ NameBuilder, Caerlun, CharacterBuilder };
use std::io;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;

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
            .takes_value(false)
            .help("If set, generate a randomly created character"))
        .arg(Arg::with_name("race")
            .short("r")
            .long("race")
            .help("Specify the race of a generated character")
            .takes_value(true))
        .arg(Arg::with_name("region")
            .short("p")
            .long("region")
            .help("Specify the region the generated character is from")
            .takes_value(true))       
        .arg(Arg::with_name("timeline")
            .short("t")
            .long("timeline")
            .help("Generate a graph showing events by time")
            .takes_value(false))     
        .arg(Arg::with_name("year")
            .short("y")
            .long("year")
            .help("Set a year for various options (e.g. start year for timeline)")
            .takes_value(true))
        .arg(Arg::with_name("regions")
            .short("m")
            .long("regions")
            .help("Generate a nested tree showing regions")
            .takes_value(false))
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
        let caerlun = build_store();
        let builder = CharacterBuilder::new(&caerlun);
        for _ in 0..count {
            let character = builder.build(
                matches.value_of("key"),
                None,
                matches.value_of("race"),
                matches.value_of("region"),
                matches.value_of("dob"));
            print!("{}", character);
        }
    } else if matches.is_present("timeline") {
        let caerlun = build_store();
        caerlun.timeline(matches.value_of("year"), None);
    } else if let Some(key) = matches.value_of("key") {
        let builder = NameBuilder::new();
        for _ in 0..count {
            println!("{}", builder.name(key));        
        }
    } else if matches.is_present("regions") {
        let caerlun = build_store();
        caerlun.print_regions();
    } else {
        let builder = NameBuilder::new();
        let mut keys = builder.keys();
        keys.sort();
        for s in keys.iter() {
            println!("{}", s);
        }
    }

    Ok(())
}

fn build_store() -> Caerlun {
    let mut caerlun = Caerlun::new();
    for p in Asset::iter() {
        if p.ends_with(".yaml") {
            // println!("About to load: {:?}", p);
            let o = Asset::get(&p);
            match o {
                Some(cow) => {
                    match std::str::from_utf8(&cow.data) {
                        Ok(s) => caerlun.build_type(s),
                        _ => (),
                    }
                },
                None => (),
            }
        }
    }
    caerlun.find_leaves();
    caerlun
}
