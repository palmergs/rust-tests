#[derive(Debug)]
pub struct Geo {
    pub key: String,
    pub name: String,
}

impl Geo {
    pub fn new(key: &str, name: &str) -> Geo {
        Geo {
            key: key.to_string(),
            name: name.to_string(),
        }
    }
}

impl PartialEq for Geo {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Geo {}
