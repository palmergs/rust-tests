use super::{
    Caerlun, NameBuilder, Race, Region, Value, ATTRIBUTES, AWA, BODY, CHR, CURRENT_YEAR, DEX, END,
    FOCUS, HEC, INT, POINTS, SPELL, STR, WIL,
};

use rand::Rng;
use std::fmt;

// use indexmap::IndexSet;

pub struct Character {
    pub name: String,
    pub family: Option<String>,
    pub nickname: Option<String>,
    pub race: (String, String),
    pub region: (String, String),
    pub dob: i64,
    pub points: [Value; POINTS],
    pub attributes: [Value; ATTRIBUTES],
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(family) = &self.family {
            write!(f, "Name: {} {}\n", self.name, family)?;
        } else {
            write!(f, "Name: {}\n", self.name)?;
        }

        write!(f, "Race: {}\n", self.race.1)?;
        write!(f, "From: {}\n", self.region.1)?;
        write!(f, "Age: {}\n", CURRENT_YEAR - self.dob)?;
        write!(
            f,
            "BDY: {:>3}/{:<3} FOC: {:>3}/{:<3} SPP: {:>3}/{:<3}\n",
            self.points[BODY].curr,
            self.points[BODY].base,
            self.points[FOCUS].curr,
            self.points[FOCUS].base,
            self.points[SPELL].curr,
            self.points[SPELL].base,
        )?;
        write!(
            f,
            "STR: {:<4} END: {:<4} DEX: {:<4} HEC: {:<4}\n",
            self.attributes[STR].curr,
            self.attributes[END].curr,
            self.attributes[DEX].curr,
            self.attributes[HEC].curr,
        )?;
        write!(
            f,
            "AWA: {:<4} INT: {:<4} WIL: {:<4} CHR: {:<4}\n",
            self.attributes[AWA].curr,
            self.attributes[INT].curr,
            self.attributes[WIL].curr,
            self.attributes[CHR].curr,
        )?;

        write!(f, "\n")
    }
}

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
        fname_key: Option<&str>,
        lname_key: Option<&str>,
        race_key: Option<&str>,
        region_key: Option<&str>,
        dob: Option<&str>,
    ) -> Character {
        let race = self.race(race_key);
        let year = self.dob(dob, &race);

        let region = self.region(region_key, Some(&race.key), year);

        let name = self.name(fname_key, &race);
        let family = self.family(lname_key, &race);

        Character {
            name: name,
            family: family,
            nickname: None,
            race: (race.key.to_string(), race.name.to_string()),
            region: (region.key.to_string(), region.name.to_string()),
            dob: year,
            points: race.points.clone(),
            attributes: race.attributes.clone(),
        }
    }

    fn dob(&self, dob: Option<&str>, race: &Race) -> i64 {
        match dob {
            Some(s) => s.parse::<i64>().expect("Parsable string for integer"),
            None => {
                let mut rng = rand::thread_rng();
                let range = &race.lifespan[1];
                let age = rng.gen_range(range.start, range.end) as i64;
                CURRENT_YEAR - age
            }
        }
    }

    fn race(&self, race_key: Option<&str>) -> &Race {
        match race_key {
            Some(s) => {
                if let Some(race) = self.store.race(s) {
                    race
                } else {
                    self.store.race("human").unwrap()
                }
            }
            None => {
                let key = Race::random_player_race();
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

    fn name(&self, name_key: Option<&str>, race: &Race) -> String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 2) {
            0 => self
                .select_name(name_key, Some(&race.mname))
                .expect("Expected fname 1"),
            1 => self
                .select_name(name_key, Some(&race.fname))
                .expect("Expected fname 2"),
            _ => panic!("Expected only 2 options"),
        }
    }

    fn family(&self, name_key: Option<&str>, race: &Race) -> Option<String> {
        self.select_name(name_key, race.lname.as_deref())
    }

    fn select_name(&self, name_key: Option<&str>, backup_key: Option<&str>) -> Option<String> {
        match name_key {
            Some(s) => Some(self.names.name(s)),
            None => match backup_key {
                Some(s) => Some(self.names.name(s)),
                None => None,
            },
        }
    }
}
