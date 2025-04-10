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
