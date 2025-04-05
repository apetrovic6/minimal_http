use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, Default)]
pub enum Status {
    #[default]
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NotFound = 404,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let status_code = *self as i32;
        match self {
            Status::Ok => write!(f, "{} OK", status_code),
            Status::Created => write!(f, "{} Created", status_code),
            Status::Accepted => write!(f, "{} Accepted", status_code),
            Status::NotFound => write!(f, "{} Not Found", status_code),
        }
    }
}
