use super::{ Caerlun, Race };
use std::str::FromStr;

use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub enum Tone {
    Positive,
    Neutral,
    Negative,
}

impl Tone {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref ALIAS_KEY: Yaml = Yaml::from_str("tone");
        }
        &ALIAS_KEY
    }
}

impl FromStr for Tone {
    type Err = ();

    fn from_str(s: &str) -> Result<Tone, ()> {
        match s {
            "positive" => Ok(Tone::Positive),
            "neutral" => Ok(Tone::Neutral),
            "negative" => Ok(Tone::Negative),
            _ => Ok(Tone::Neutral),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub tone: Tone,
    pub races: Vec<String>,
}

impl Alias {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref ALIAS_KEY: Yaml = Yaml::from_str("alias");
        }
        &ALIAS_KEY
    }

    pub fn build(yaml: Option<&Yaml>) -> Vec<Alias> {
        match yaml {
            Some(yaml) => match yaml {
                Yaml::Array(arr) => {
                    let mut vec = Vec::new();
                    for a in arr {
                        match Alias::build_one(a) {
                            Some(alias) => vec.push(alias),
                            None => (),
                        }
                    }
                    vec
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    }

    pub fn build_one(yaml: &Yaml) -> Option<Alias> {
        match yaml {
            Yaml::Hash(h) => {
                let tone = match h.get(Tone::key()) {
                    Some(s) => Tone::from_str(s.as_str().unwrap()).unwrap(),
                    None => Tone::Neutral,
                };

                let alias = Alias {
                    name: h[Caerlun::name_key()].as_str().unwrap().to_string(),
                    tone: tone,
                    races: Caerlun::strings(h.get(Race::key())),
                };

                Some(alias)
            }
            _ => None,
        }
    }
}
