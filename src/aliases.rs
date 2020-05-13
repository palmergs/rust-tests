use std::str::FromStr;

#[derive(Debug)]
pub enum Tone {
    Positive,
    Neutral,
    Negative,
}

impl FromStr for Tone {
    type Err = ();

    fn from_str(s: &str) -> Result<Tone, ()> {
        match s {
            "positive" => Ok(Tone::Positive),
            "neutral" => Ok(Tone::Neutral),
            "negative" => Ok(Tone::Negative),
            _ => Ok(Tone::Neutral),
        }
    }
}

#[derive(Debug)]
pub struct Alias<'a> {
    name: &'a str,
    tone: Tone,
    races: Vec<u32>,
}