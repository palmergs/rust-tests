#[macro_use] extern crate lazy_static;

extern crate sorted_vec;
extern crate yaml_rust;
extern crate indexmap;

use rust_embed::RustEmbed;

mod names;
pub use names::{ NameBuilder, Fragment, FragmentList };

mod articles;
pub use articles::{ Caerlun, Region, Race, Timeline, GeoFeature };

pub mod characters;
pub use characters::{ CharacterBuilder };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;


