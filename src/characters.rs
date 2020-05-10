use super::Caerlun;

use rand::seq::SliceRandom;
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

        let mut rng = rand::thread_rng(); 
        match race_key {
            Some(s) => {
                println!("Race: {}", s); 
            },
            None => {
                let n = self.store.races.len();
                let (_, race) = self.store.races.get_index(rng.gen_range(0, n)).unwrap();
                println!("Race: {}", race.name);
            },
        }
        println!("Region: {:?}", region_key);
        println!("DOB: {:?}", dob);
    }

}

