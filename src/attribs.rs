use yaml_rust::Yaml;

use super::Caerlun;

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub bdy: i16,
    pub foc: i16,
}

impl Stats {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref STATS_KEY: Yaml = Yaml::from_str("stats");
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

    pub fn build(yaml: &Yaml) -> Stats {
        match yaml {
            Yaml::Hash(h) => Stats {
                bdy: Caerlun::opt_integer(h.get(Stats::bdy_key()))
                    .expect("missing bdy attribute") as i16,
                foc: Caerlun::opt_integer(h.get(Stats::foc_key()))
                    .expect("missing foc attribute") as i16,
            },
            _ => panic!("Expected has for stats"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attribs {
    pub st: i8,
    pub en: i8,
    pub dx: i8,
    pub aw: i8,
    pub hc: i8,
    pub it: i8,
    pub wi: i8,
    pub ch: i8,
}

impl Attribs {
    pub fn key() -> &'static Yaml {
        lazy_static! {
            static ref ATTRIBS_KEY: Yaml = Yaml::from_str("attributes");
        }
        &ATTRIBS_KEY
    }

    pub fn st_key() -> &'static Yaml {
        lazy_static! {
            static ref ST_KEY: Yaml = Yaml::from_str("str");
        }
        &ST_KEY
    }

    pub fn en_key() -> &'static Yaml {
        lazy_static! {
            static ref EN_KEY: Yaml = Yaml::from_str("end");
        }
        &EN_KEY
    }

    pub fn dx_key() -> &'static Yaml {
        lazy_static! {
            static ref DX_KEY: Yaml = Yaml::from_str("dex");
        }
        &DX_KEY
    }

    pub fn aw_key() -> &'static Yaml {
        lazy_static! {
            static ref AW_KEY: Yaml = Yaml::from_str("awa");
        }
        &AW_KEY
    }

    pub fn hc_key() -> &'static Yaml {
        lazy_static! {
            static ref HC_KEY: Yaml = Yaml::from_str("hec");
        }
        &HC_KEY
    }

    pub fn it_key() -> &'static Yaml {
        lazy_static! {
            static ref IT_KEY: Yaml = Yaml::from_str("int");
        }
        &IT_KEY
    }

    pub fn wi_key() -> &'static Yaml {
        lazy_static! {
            static ref WI_KEY: Yaml = Yaml::from_str("wil");
        }
        &WI_KEY
    }

    pub fn ch_key() -> &'static Yaml {
        lazy_static! {
            static ref CH_KEY: Yaml = Yaml::from_str("chr");
        }
        &CH_KEY
    }

    pub fn build(yaml: &Yaml) -> Attribs {
        match yaml {
            Yaml::Hash(h) => Attribs {
                st: Caerlun::opt_integer(h.get(Attribs::st_key()))
                    .expect("missing str attribute") as i8,
                en: Caerlun::opt_integer(h.get(Attribs::en_key()))
                    .expect("missing end attribute") as i8,
                dx: Caerlun::opt_integer(h.get(Attribs::dx_key()))
                    .expect("missing dex attribute") as i8,
                hc: Caerlun::opt_integer(h.get(Attribs::hc_key()))
                    .expect("missing hec attribute") as i8,
                aw: Caerlun::opt_integer(h.get(Attribs::aw_key()))
                    .expect("missing awa attribute") as i8,
                it: Caerlun::opt_integer(h.get(Attribs::it_key()))
                    .expect("missing int attribute") as i8,
                wi: Caerlun::opt_integer(h.get(Attribs::wi_key()))
                    .expect("missing wil attribute") as i8,
                ch: Caerlun::opt_integer(h.get(Attribs::ch_key()))
                    .expect("missing chr attribute") as i8,
            },
            _ => panic!("Expected attribute hash"),
        }
    }
}
