use std::cmp::Ordering;
use std::ops::Range;
use super::Alias;

#[derive(Debug)]
pub struct Era<'a> {
    pub key: &'a str,
    pub abbr: &'a str,
    pub name: &'a str,
    pub range: Range<u32>,
    pub alias: Vec<Alias<'a>>,
    pub races: Vec<u32>
}

impl<'a> Ord for Era<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl<'a> PartialOrd for Era<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Era<'a> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<'a> Eq for Era<'a> {}