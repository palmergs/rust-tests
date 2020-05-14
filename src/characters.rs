use super::{ NameBuilder, Caerlun, Race, Region };

use rand::Rng;

pub struct Character {}

pub struct CharacterBuilder<'a> {
    store: &'a Caerlun,
    names: NameBuilder,
    pc_race_keys: Vec<&'a str>,
}

impl<'a> CharacterBuilder<'a> {
    pub fn new(store: &'a Caerlun) -> CharacterBuilder<'a> {
        CharacterBuilder { 
            store: store,
            names: NameBuilder::new(),
            pc_race_keys: vec!["human", "elf", "dwarf", "rulligg", "feletaur", "centaur", "urunai", "gobru", "urg"],
        }
    }

    pub fn build(
        &self,
        name_key: Option<&str>,
        race_key: Option<&str>,
        region_key: Option<&str>,
        dob: Option<&str>) {

        let race = self.race(race_key);
        let region = self.region(region_key, &race.key);
        let name = self.name(name_key, &race.key);

        println!("Name: {}", name);
        println!("Race: {}", race.name);
        println!("Region: {}", region.name);
        println!("DOB: {:?}", dob);
    }

    fn race(&self, race_key: Option<&str>) -> &Race {
        let mut rng = rand::thread_rng(); 
        match race_key {
            Some(s) => {
                if let Some(race) = self.store.race(s) {
                    race
                } else {
                    self.store.race("human").unwrap()
                }
            },
            None => {
                let n = self.pc_race_keys.len();
                let key = self.pc_race_keys[rng.gen_range(0, n)];
                self.store.race(key).unwrap()
            }
        }
    }

    fn region(&self, region_key: Option<&str>, race_key: &str) -> &Region {
        let mut rng = rand::thread_rng();
        match region_key {
            Some(s) => {
                if let Some(region) = self.store.region(s) {
                    region
                } else {
                    self.store.region("opal").unwrap()
                }
            },
            None => {
                let n = self.store.regions.len();
                let (_, region) = self.store.regions.get_index(rng.gen_range(0, n)).unwrap();
                region
            }
        }
    }

    fn name(&self, name_key: Option<&str>, race_key: &str) -> String {
        match name_key {
            Some(s) => {
                self.names.name(s)
            },
            None => {
                self.names.name(race_key)
            }
        }
    }
}

