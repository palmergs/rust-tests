use super::{ Caerlun, Race, Region };

use rand::Rng;

pub struct Character {}

pub struct CharacterBuilder<'a> {
    store: &'a Caerlun<'a>,
}

impl<'a> CharacterBuilder<'a> {
    pub fn new(store: &'a Caerlun) -> CharacterBuilder<'a> {
        CharacterBuilder { store: store }
    }

    pub fn build(
        &self,
        name_key: Option<&str>,
        race_key: Option<&str>,
        region_key: Option<&str>,
        dob: Option<&str>) {

        println!("Name: {:?}", name_key);

        let race = self.race(race_key);
        println!("Race: {}", race.name);

        let region = self.region(region_key, &race.id);
        println!("Region: {}", region.name);

        println!("DOB: {:?}", dob);
    }

    fn race(&self, race_key: Option<&str>) -> &Race {
        let mut rng = rand::thread_rng(); 
        match race_key {
            Some(s) => {
                if let Some(race) = self.store.races.get(s) {
                    race
                } else {
                    self.store.races.get("human").unwrap()
                }
            },
            None => {
                let n = self.store.races.len();
                let (_, race) = self.store.races.get_index(rng.gen_range(0, n)).unwrap();
                race
            }
        }
    }

    fn region(&self, region_key: Option<&str>, race_key: &str) -> &Region {
        let mut rng = rand::thread_rng();
        match region_key {
            Some(s) => {
                if let Some(region) = self.store.regions.get(s) {
                    region
                } else {
                    self.store.regions.get("opal").unwrap()
                }
            },
            None => {
                let n = self.store.regions.len();
                let (_, region) = self.store.regions.get_index(rng.gen_range(0, n)).unwrap();
                region
            }
        }
    }
}

