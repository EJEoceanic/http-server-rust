use std::collections::HashMap;

pub const PROTOCOL_VERSION: &str = "HTTP/1.1";

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HTTPMethod {
    pub fn from_string(word: &str) -> Result<HTTPMethod, anyhow::Error> {
        match word {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "DELETE" => Ok(HTTPMethod::DELETE),
            _ => Err(anyhow::format_err!("Invalid method")),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::POST => "POST",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::PATCH => "PATCH",
            HTTPMethod::DELETE => "DELETE",
        }
    }
}

pub enum Status {
    Ok = 200,
    NotFound = 404,
}

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
        }
    }

    pub fn new(code: isize) -> Option<Status> {
        match code {
            200 => Some(Status::Ok),
            404 => Some(Status::NotFound),
            _ => None,
        }
    }
}
pub struct Header {
    protocol_version: String,
    method: HTTPMethod,
    status: Status,
    path: String,
    data: HashMap<String, String>,
}

impl Header {
    pub fn new() -> Header {
        Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            method: HTTPMethod::GET,
            status: Status::Ok,
            path: "".to_string(),
            data: HashMap::new(),
        }
    }

    pub fn from_string(header_data: &str) -> Result<Header, anyhow::Error> {
        let mut new_header = Header::new();
        let mut lines = header_data.lines();

        new_header.parse_status_line(lines.nth(0))?;

        for line in lines {
            if let Some((key, value)) = line.split_once(":") {
                new_header.data.insert(key.to_string(), value.to_string());
            }
        }

        Ok(new_header)
    }

    pub fn parse_status_line(&mut self, line_option: Option<&str>) -> Result<(), anyhow::Error> {
        match line_option {
            Some(line) => {
                let mut data = line.split_whitespace();
                self.method = HTTPMethod::from_string(data.next().unwrap())?;
                self.path = data.next().unwrap_or_else(|| "/").to_string();
                self.protocol_version = data.next().unwrap_or_else(|| PROTOCOL_VERSION).to_string();
                Ok(())
            }
            None => Err(anyhow::format_err!("There is no status line")),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path.as_str()
    }
}
