#[macro_use] extern crate lazy_static;

use rust_embed::RustEmbed;
use std::collections::hash_map::HashMap;

mod generator;
pub use generator::{ Fragment, FragmentList };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;

#[derive(Debug)]
pub struct NameBuilder {
    hash: HashMap<String, FragmentList>,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        let mut builder = NameBuilder { hash: HashMap::new() };
        for p in Asset::iter() {
            let o = Asset::get(&p);
            match o {
                Some(cow) => {
                    match std::str::from_utf8(&cow) {
                        Ok(s) => builder.parse(s),
                        _ => ()
                    }
                }
                None => ()
            };
        }
        return builder
    }

    fn parse(&mut self, contents: &str) {
        let mut curr_ident = "".to_string();
        for line in contents.lines() {
            if line != "" {
                let head = FragmentList::get_header(line);
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
                                frag.add(f);
                            } else {
                                let fs = vec.iter().map(|&x| Fragment::new(x)).collect::<Vec<_>>();
                                frag.add(Fragment::Series(fs));
                            }
                        },
                        None => (),
                    }
                }
            }
        }
    }

    pub fn name(&self, key: &str) -> String {
        match &self.hash.get(key) {
            Some(fragment_list) => {
                match fragment_list.choose() {
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

}
