use super::{Attribs, Caerlun, Event, NameBuilder, Race, Region, Stats};

use rand::Rng;
use std::fmt;

static CURRENT_YEAR: i64 = 1260;

pub struct Character {
    pub fname: String,
    pub lname: Option<String>,
    pub nickname: Option<String>,
    pub race: (String, String),
    pub region: (String, String),
    pub dob: i64,
    pub max_stat: Stats,
    pub cur_stat: Stats,
    pub max_atts: Attribs,
    pub cur_atts: Attribs,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(last) = &self.lname {
            write!(f, "Name: {} {}\n", self.fname, last)?;
        } else {
            write!(f, "Name: {}\n", self.fname)?;
        }

        write!(f, "Race: {} from {}\n", self.race.1, self.region.1)?;
        write!(f, "Age: {}\n", CURRENT_YEAR - self.dob)?;
        write!(
            f,
            "BDY: {:>3}/{:<3} FOC: {:>3}/{:<3}\n",
            self.cur_stat.bdy, self.max_stat.bdy, self.cur_stat.foc, self.max_stat.foc
        )?;
        write!(
            f,
            "STR: {:<4} END: {:<4} DEX: {:<4} HEC: {:<4}\n",
            self.cur_atts.st, self.cur_atts.en, self.cur_atts.dx, self.cur_atts.hc
        )?;
        write!(
            f,
            "AWA: {:<4} INT: {:<4} WIL: {:<4} CHR: {:<4}\n",
            self.cur_atts.aw, self.cur_atts.it, self.cur_atts.wi, self.cur_atts.ch
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

        let fname = self.fname(fname_key, &race);
        let lname = self.lname(lname_key, &race);
        let events = self.events_from(&region.key, year, CURRENT_YEAR);

        let mut stats = race.stats.clone();
        let mut atts = race.atts.clone();
        CharacterBuilder::modify(&mut stats, &mut atts);

        Character {
            fname: fname,
            lname: lname,
            nickname: None,
            race: (race.key.to_string(), race.name.to_string()),
            region: (region.key.to_string(), region.name.to_string()),
            dob: year,
            max_stat: stats,
            cur_stat: stats,
            max_atts: atts,
            cur_atts: atts,
        }
    }

    // Randomly modify the stats 
    fn modify(stats: &mut Stats, atts: &mut Attribs) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 4) {
            0 => {
                // Strong
                stats.bdy = stats.bdy + rng.gen_range(0, 3);
                atts.st = atts.st + 1;
                atts.en = atts.en + 1;
                if rng.gen_range(0, 2) == 1 {
                    atts.it = atts.it - 1;
                    atts.wi = atts.wi - 1;
                }
            },
            1 => {
                // Smart
                stats.foc = stats.foc + rng.gen_range(0, 3);
                atts.it = atts.it + 1;
                atts.aw = atts.aw + 1;
                if rng.gen_range(0, 2) == 1 {
                    atts.st = atts.st - 1;
                    atts.en = atts.en - 1;
                }
            },
            2 => {
                // Fast
                stats.foc = stats.foc + rng.gen_range(0, 2);
                atts.dx = atts.dx + 1;
                atts.hc = atts.hc + 1;
                if rng.gen_range(0, 2) == 1 {
                    atts.st = atts.st - 1;
                    atts.en = atts.en - 1;
                }
            },
            3 => {
                // Wise
                stats.foc = stats.foc + rng.gen_range(0, 2);
                atts.wi = atts.wi + 1;
                atts.ch = atts.ch + 1;
                if rng.gen_range(0, 2) == 1 {
                    atts.st = atts.st - 1;
                    atts.it = atts.it - 1;
                }
            },
            _ => ()
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

    fn fname(&self, name_key: Option<&str>, race: &Race) -> String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 2) {
            0 => self
                .name(name_key, Some(&race.mname))
                .expect("Expected fname 1"),
            1 => self
                .name(name_key, Some(&race.fname))
                .expect("Expected fname 2"),
            _ => panic!("Expected only 2 options"),
        }
    }

    fn lname(&self, name_key: Option<&str>, race: &Race) -> Option<String> {
        self.name(name_key, race.lname.as_deref())
    }

    fn name(&self, name_key: Option<&str>, backup_key: Option<&str>) -> Option<String> {
        match name_key {
            Some(s) => Some(self.names.name(s)),
            None => match backup_key {
                Some(s) => Some(self.names.name(s)),
                None => None,
            },
        }
    }

    fn events_from(&self, region_key: &str, from: i64, to: i64) -> Vec<Event> {
        let mut rng = rand::thread_rng();

        let idx = rng.gen_range(from, to);

        Vec::new()
    }
}
