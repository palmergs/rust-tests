#[macro_use] extern crate lazy_static;

extern crate sorted_vec;
extern crate yaml_rust;
extern crate indexmap;
extern crate nested_intervals;
extern crate rust_embed;

use rust_embed::RustEmbed;

mod names;
pub use names::{ NameBuilder, Fragment, FragmentList };

mod articles;
pub use articles::{ Caerlun };

mod aliases;
pub use aliases::{ Tone, Alias };

mod races;
pub use races::{ Race };

mod regions;
pub use regions::{ Region };

mod events;
pub use events::{ parse_years, Event };

mod features;
pub use features::{ Geo };

pub mod characters;
pub use characters::{ CharacterBuilder };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;


