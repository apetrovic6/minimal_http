use crate::user_agent;

use super::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    // pub body: String,
}

#[derive(Debug)]
pub struct ReqError {
    pub msg: String,
}

impl Request {
    fn parse_method_and_path(strings: Vec<&str>) -> Result<(String, Method), ReqError> {
        let [method, path, _]: [_; 3] = strings.try_into().ok().unwrap();

        let method = match method.parse::<Method>() {
            Ok(m) => m,
            Err(_) => {
                return Err(ReqError {
                    msg: String::from("Wrong Error"),
                })
            }
        };

        let path = path.split('/').collect::<Vec<&str>>().join("/");

        Ok((path, method))
    }

    fn parse_host(strings: Vec<&str>) -> &str {
        strings.get(1).unwrap_or(&"")
    }

    fn parse_user_agent(strings: Vec<&str>) -> &str {
        strings.get(1).unwrap_or(&"")
    }

    fn parse_accept(strings: Vec<&str>) -> &str {
        strings.get(1).unwrap_or(&"")
    }
}

impl TryFrom<Vec<String>> for Request {
    type Error = ReqError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let method_path = value.first();
        let method_path = method_path.unwrap();

        let method_path: Vec<&str> = method_path.split(' ').collect();

        let (path, method) = Self::parse_method_and_path(method_path)?;

        let host = match value.get(1) {
            Some(s) => s,
            None => &String::from(""),
        };

        let host = Self::parse_host(host.split(" ").collect());

        let binding = value
            .iter()
            .filter(|s| s.contains("User-Agent"))
            .collect::<Vec<_>>();

        let a = binding.get(0);

        let user_agent: String = match binding.first() {
            Some(s) => s.split_whitespace().last().unwrap().to_string(),
            None => String::new(),
        };

        // let user_agent = Self::parse_user_agent(user_agent.split(" ").collect());

        println!("Parsed user_agent: {:?}", user_agent);

        let accept = match value.get(3) {
            Some(s) => s,
            None => &String::from(""),
        };

        let accept = Self::parse_accept(accept.split(" ").collect());

        Ok(Self {
            method,
            path: path.to_string(),
            host: String::from(host),
            user_agent: user_agent,
            accept: String::from(accept),
        })
    }
}
