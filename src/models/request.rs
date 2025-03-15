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

        let host = value
            .iter()
            .find(|s| s.contains("Host"))
            .and_then(|s| s.split_whitespace().last())
            .map(ToString::to_string)
            .unwrap_or_default();

        let user_agent = value
            .iter()
            .find(|s| s.contains("User-Agent"))
            .and_then(|s| s.split_whitespace().last())
            .map(ToString::to_string)
            .unwrap_or_default();

        println!("Parsed host: {:?}", host);

        let accept = match value.get(3) {
            Some(s) => s,
            None => &String::from(""),
        };

        let accept = Self::parse_accept(accept.split(" ").collect());

        Ok(Self {
            method,
            path: path.to_string(),
            host,
            user_agent,
            accept: String::from(accept),
        })
    }
}
