use std::str::FromStr;
use strum::Display;

#[derive(Display, Debug, Hash, Eq, PartialEq)]
pub enum EncodingType {
    #[strum(to_string = "gzip")]
    Gzip,
    None,
}

// impl EncodingType {
//     fn description(&self) -> &'static str {
//         match self {
//             EncodingType::Gzip => "gzip",
//             EncodingType::None => "",
//         }
//     }
// }
//
impl FromStr for EncodingType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gzip" => Ok(EncodingType::Gzip),
            _ => Ok(EncodingType::None),
        }
    }
}
//
// impl Display for EncodingType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.description())
//     }
// }
