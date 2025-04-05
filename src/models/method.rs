use strum::EnumString;

#[derive(Debug, Hash, Eq, PartialEq, Default, EnumString)]
pub enum Method {
    #[default]
    #[strum(serialize = "GET", ascii_case_insensitive)]
    Get,
    #[strum(serialize = "POST", ascii_case_insensitive)]
    Post,
    #[strum(serialize = "PATCH", ascii_case_insensitive)]
    Patch,
    #[strum(serialize = "PUT", ascii_case_insensitive)]
    Put,
    #[strum(serialize = "DELETE", ascii_case_insensitive)]
    Delete,
}

// impl FromStr for Method {
//     type Err = &'static str;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "GET" => Ok(Method::Get),
//             "POST" => Ok(Method::Post),
//             "PUT" => Ok(Method::Put),
//             "PATCH" => Ok(Method::Patch),
//             "DELETE" => Ok(Method::Delete),
//             _ => Err("Wrong method type"),
//         }
//     }
// }
