use yaml_rust::Yaml;

use super::Caerlun;

pub const POINTS: usize = 3;
pub const BODY: usize = 0;
pub const FOCUS: usize = 1;
pub const SPELL: usize = 2;

pub const ATTRIBUTES: usize = 8;
pub const STR: usize = 0;
pub const END: usize = 1;
pub const DEX: usize = 2;
pub const AWA: usize = 3;
pub const HEC: usize = 4;
pub const INT: usize = 5;
pub const WIL: usize = 6;
pub const CHR: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub base: i16,
    pub mods: i16,
    pub lost: i16,
    pub temp: i16,
    pub curr: i16,
}

impl Value {
    pub fn attributes_key() -> &'static Yaml {
        lazy_static! {
            static ref ATTRIBS_KEY: Yaml = Yaml::from_str("attributes");
        }
        &ATTRIBS_KEY
    }

    pub fn points_key() -> &'static Yaml {
        lazy_static! {
            static ref STATS_KEY: Yaml = Yaml::from_str("points");
        }
        &STATS_KEY
    }

    pub fn bdy_key() -> &'static Yaml {
        lazy_static! {
            static ref BDY_KEY: Yaml = Yaml::from_str("bdy");
        }
        &BDY_KEY
    }

    pub fn foc_key() -> &'static Yaml {
        lazy_static! {
            static ref FOC_KEY: Yaml = Yaml::from_str("foc");
        }
        &FOC_KEY
    }

    pub fn spp_key() -> &'static Yaml {
        lazy_static! {
            static ref SPP_KEY: Yaml = Yaml::from_str("spp");
        }
        &SPP_KEY
    }

    pub fn str_key() -> &'static Yaml {
        lazy_static! {
            static ref STR_KEY: Yaml = Yaml::from_str("str");
        }
        &STR_KEY
    }

    pub fn end_key() -> &'static Yaml {
        lazy_static! {
            static ref END_KEY: Yaml = Yaml::from_str("end");
        }
        &END_KEY
    }

    pub fn dex_key() -> &'static Yaml {
        lazy_static! {
            static ref DEX_KEY: Yaml = Yaml::from_str("dex");
        }
        &DEX_KEY
    }

    pub fn awa_key() -> &'static Yaml {
        lazy_static! {
            static ref AWA_KEY: Yaml = Yaml::from_str("awa");
        }
        &AWA_KEY
    }

    pub fn hec_key() -> &'static Yaml {
        lazy_static! {
            static ref HEC_KEY: Yaml = Yaml::from_str("hec");
        }
        &HEC_KEY
    }

    pub fn int_key() -> &'static Yaml {
        lazy_static! {
            static ref INT_KEY: Yaml = Yaml::from_str("int");
        }
        &INT_KEY
    }

    pub fn wil_key() -> &'static Yaml {
        lazy_static! {
            static ref WIL_KEY: Yaml = Yaml::from_str("wil");
        }
        &WIL_KEY
    }

    pub fn chr_key() -> &'static Yaml {
        lazy_static! {
            static ref CHR_KEY: Yaml = Yaml::from_str("chr");
        }
        &CHR_KEY
    }

    pub fn build_points(yaml: &Yaml) -> Vec<Value> {
        match yaml {
            Yaml::Hash(h) => vec![
                Value::build_value(h.get(Value::bdy_key())),
                Value::build_value(h.get(Value::foc_key())),
                Value::build_value(h.get(Value::spp_key())),
            ],
            _ => panic!("Expected attribute hash"),
        }
    }

    pub fn build_attributes(yaml: &Yaml) -> Vec<Value> {
        match yaml {
            Yaml::Hash(h) => vec![
                Value::build_value(h.get(Value::str_key())),
                Value::build_value(h.get(Value::end_key())),
                Value::build_value(h.get(Value::dex_key())),
                Value::build_value(h.get(Value::hec_key())),
                Value::build_value(h.get(Value::awa_key())),
                Value::build_value(h.get(Value::int_key())),
                Value::build_value(h.get(Value::wil_key())),
                Value::build_value(h.get(Value::chr_key())),
            ],
            _ => panic!("Expected attribute hash"),
        }
    }

    fn build_value(opt: Option<&Yaml>) -> Value {      
        match Caerlun::opt_integer(opt) {
            Some(val) => Value { base: val as i16, mods: 0, lost: 0, temp: 0, curr: val as i16 },
            None => Value { base: 0, mods: 0, lost: 0, temp: 0, curr: 0 }
        }
    }
}
