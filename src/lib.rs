#[macro_use] extern crate lazy_static;

extern crate sorted_vec;
extern crate yaml_rust;

use rust_embed::RustEmbed;

mod names;
pub use names::{ NameBuilder, Fragment, FragmentList };

mod articles;
pub use articles::{ Caerlun, Region, Timeline, GeoFeature };

pub mod characters;
pub use characters::{ CharacterBuilder };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;


