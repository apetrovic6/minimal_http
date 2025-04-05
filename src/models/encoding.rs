use std::str::FromStr;
use strum::Display;

#[derive(Display, Debug, Hash, Eq, PartialEq)]
pub enum EncodingType {
    #[strum(to_string = "gzip")]
    Gzip,
    None,
}

impl FromStr for EncodingType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gzip" => Ok(EncodingType::Gzip),
            _ => Ok(EncodingType::None),
        }
    }
}
