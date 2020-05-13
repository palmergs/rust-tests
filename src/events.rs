use std::cmp::Ordering;
use std::ops::Range;
use super::Alias;

#[derive(Debug)]
pub struct Event<'a> {
    pub key: &'a str,
    pub name: &'a str,
    pub range: Range<i64>,
    pub alias: Vec<Alias<'a>>,
    pub races: Vec<usize>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<'a> Ord for Event<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl<'a> PartialOrd for Event<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Event<'a> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<'a> Eq for Event<'a> {}