use strum::Display;

#[derive(Debug, Copy, Clone, Default, Display)]
pub enum Status {
    #[default]
    #[strum(to_string = "200 OK")]
    Ok = 200,
    #[strum(to_string = "201 Created")]
    Created = 201,
    #[strum(to_string = "202 Accepted")]
    Accepted = 202,
    #[strum(to_string = "404 Not Found")]
    NotFound = 404,
}
