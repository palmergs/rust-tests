extern crate clap;
use rust_names::NameBuilder;
use std::fs;
use std::path::Path;
use std::io;

use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let matches = App::new("randomlines")
        .version(VERSION)
        .about("Reads random lines from a file.!")
        .author("Galen P.")
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .help("Directory where resources are found")
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

    let path = matches.value_of("dir").unwrap_or("resources");
    let path = Path::new(path);
    let mut builder = NameBuilder::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                match path.extension() {
                    Some(ex) => {
                        if ex == "txt" {
                            let contents = fs::read_to_string(&path)?;
                            match builder.parse(&contents) {
                                Ok(_) => (),
                                Err(msg) => println!("Unable to parse file{:?}: err={}", &path, msg),
                            }
                        }
                    },
                    None => ()
                }
            }
        }
    }

    let key = matches.value_of("key").unwrap_or("");
    if key == "" {
        display_keys(&builder);
    } else {
        let count_str = matches.value_of("count").unwrap_or("100");
        let count = count_str.parse::<i32>().unwrap();
        display_names(&builder, count, key);
    }

    Ok(())
}

fn display_keys(builder: &NameBuilder) {
    for s in builder.keys().iter() {
        println!("{}", s);
    }
}

fn display_names(builder: &NameBuilder, count: i32, key: &str) {
    for _ in 0..count {
        println!("{}", builder.name(key));
    }
}
