use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, Default)]
pub enum Status {
    #[default]
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NotFound = 404,
}

impl Status {
    fn description(&self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::Created => "Created",
            Status::Accepted => "Accepted",
            Status::NotFound => "Not Found",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", *self as i32, self.description())
    }
}
