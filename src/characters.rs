
pub struct Character {}

pub struct CharacterBuilder {}

impl CharacterBuilder {
    pub fn new() -> CharacterBuilder {
        CharacterBuilder {}
    }

    pub fn build(
        &self,
        name_key: Option<&str>,
        race_key: Option<&str>,
        region_key: Option<&str>,
        dob: Option<&str>) {
    }
}

