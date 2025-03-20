use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            _ => Err("Wrong method type"),
        }
    }
}
