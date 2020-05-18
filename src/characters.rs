use super::{Caerlun, Event, NameBuilder, Race, Region};

use rand::Rng;

static CURRENT_YEAR: i64 = 840;

pub struct Character {}

pub struct CharacterBuilder<'a> {
    store: &'a Caerlun,
    names: NameBuilder,
}

impl<'a> CharacterBuilder<'a> {
    pub fn new(store: &'a Caerlun) -> CharacterBuilder<'a> {
        CharacterBuilder {
            store: store,
            names: NameBuilder::new(),
        }
    }

    pub fn build(
        &self,
        name_key: Option<&str>,
        race_key: Option<&str>,
        region_key: Option<&str>,
        dob: Option<&str>,
    ) {
        let mut rng = rand::thread_rng();
        let race = self.race(race_key);
        let year = match dob {
            Some(s) => s.parse::<i64>().unwrap(),
            None => (CURRENT_YEAR - (20 + rng.gen_range(0, 20))),
        };

        let region = self.region(region_key, Some(&race.key), year);

        let name = self.name(name_key, &race.key);
        let events = self.events_from(&region.key, year, CURRENT_YEAR);

        println!("Name: {}", name);
        println!("Race: {}", race.name);
        println!("Region: {}", region.name);
        println!("DOB: {}", year);
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
            }
            None => {
                let n = Race::pc().len();
                let key = Race::pc()[rng.gen_range(0, n)];
                self.store.race(key).unwrap()
            }
        }
    }

    fn region(&self, region_key: Option<&str>, race_key: Option<&str>, dob: i64) -> &Region {
        match region_key {
            Some(s) => {
                if let Some(region) = self.store.region(s) {
                    region
                } else {
                    self.store.leaf_region(dob, race_key)
                }
            }
            None => self.store.leaf_region(dob, race_key),
        }
    }

    fn name(&self, name_key: Option<&str>, race_key: &str) -> String {
        match name_key {
            Some(s) => self.names.name(s),
            None => self.names.name(race_key),
        }
    }

    fn events_from(&self, region_key: &str, from: i64, to: i64) -> Vec<Event> {
        let mut rng = rand::thread_rng();

        let idx = rng.gen_range(from, to);

        Vec::new()
    }
}
