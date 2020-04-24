extern crate clap;
use rust_names::NameBuilder;
use std::io;
use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let matches = App::new("randomlines")
        .version(VERSION)
        .about("Reads random lines from a file.!")
        .author("Galen P.")
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

    let builder = NameBuilder::new();
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
