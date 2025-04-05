use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Display, Debug, Hash, Eq, PartialEq, EnumString)]
pub enum EncodingType {
    #[strum(to_string = "gzip")]
    Gzip,
    #[strum(to_string = "")]
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
