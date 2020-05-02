#[macro_use] extern crate lazy_static;

extern crate sorted_vec;
extern crate yaml_rust;

use rust_embed::RustEmbed;
use std::collections::hash_map::HashMap;

mod names;
pub use names::{ NameBuilder, Fragment, FragmentList };

mod articles;
pub use articles::{ Region, Timeline, GeoFeature };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;


