use std::cmp::Ordering;
use super::Alias;

#[derive(Debug)]
pub struct Race<'a> {
    pub key: &'a str,
    pub name: &'a str,
    pub plural: Option<&'a str>,
    pub alias: Vec<Alias<'a>>,
    pub regions: Vec<u32>,
}

impl<'a> PartialEq for Race<'a> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<'a> Eq for Race<'a> {}

impl<'a> Ord for Race<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.name.cmp(&other.name) }
}

impl<'a> PartialOrd for Race<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}