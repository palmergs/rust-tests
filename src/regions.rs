use std::cmp::Ordering;
use super::Alias;

#[derive(Debug)]
pub struct Region<'a> {
    pub key: &'a str,
    pub name: &'a str,
    pub plural: Option<&'a str>,
    pub alias: Vec<Alias>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<'a> Region<'a> {
    pub fn new(key: &'a str, name: &'a str) -> Region<'a> {
        Region {
            key: key,
            name: name,
            plural: None,
            alias: Vec::new(),
            parent: None,
            children: Vec::new()
        }
    }
}

impl<'a> PartialEq for Region<'a> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<'a> Eq for Region<'a> {}

impl<'a> Ord for Region<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.name.cmp(&other.name) }
}

impl<'a> PartialOrd for Region<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}