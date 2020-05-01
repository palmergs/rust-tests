#[macro_use] extern crate lazy_static;

extern crate sorted_vec;

use rust_embed::RustEmbed;
use std::collections::hash_map::HashMap;

mod generator;
pub use generator::{ NameBuilder, Fragment, FragmentList };

mod article;
pub use article::{ Region, Timeline, GeoFeature };

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;


