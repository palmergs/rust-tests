#[macro_use]
extern crate lazy_static;

extern crate indexmap;
extern crate nested_intervals;
extern crate rust_embed;
extern crate sorted_vec;
extern crate yaml_rust;

use rust_embed::RustEmbed;

mod names;
pub use names::{Fragment, FragmentList, NameBuilder};

mod articles;
pub use articles::{Caerlun, CURRENT_YEAR};

mod aliases;
pub use aliases::{Alias, Tone};

mod races;
pub use races::Race;

mod attribs;
pub use attribs::{
    Value, ATTRIBUTES, AWA, BODY, CHR, DEX, END, FOCUS, HEC, INT, POINTS, SPELL, STR, WIL,
};

mod regions;
pub use regions::Region;

mod events;
pub use events::Event;

mod features;
pub use features::Geo;

pub mod characters;
pub use characters::{Character, CharacterBuilder};

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;
