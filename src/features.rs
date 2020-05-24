use yaml_rust::Yaml;

use super::{Alias, Caerlun};

#[derive(Debug, Clone)]
pub struct Geo {
    pub key: String,
    pub name: String,
    pub alias: Vec<Alias>,
}

impl Geo {
    pub fn build(yaml: &Yaml) -> Geo {
        match yaml {
            Yaml::Hash(h) => {
                let key = Caerlun::opt_string(h.get(Caerlun::id_key())).expect("missing id key");
                let name =
                    Caerlun::opt_string(h.get(Caerlun::name_key())).expect("missing name key");
                Geo {
                    key: key.to_string(),
                    name: name.to_string(),
                    alias: Alias::build(h.get(Alias::key())),
                }
            }
            _ => panic!("Expected hash to build a geo feature"),
        }
    }
}

impl PartialEq for Geo {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Geo {}
