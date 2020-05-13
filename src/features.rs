#[derive(Debug)]
pub struct Geo<'a> {
    pub key: &'a str,
    pub name: &'a str,
}

impl<'a> Geo<'a> {
    pub fn new(key: &'a str, name: &'a str) -> Geo<'a> { 
        Geo { key: key, name: name }
    }
}

impl<'a> PartialEq for Geo<'a> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<'a> Eq for Geo<'a> {}