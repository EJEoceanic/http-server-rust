use std::collections::HashMap;

use super::encoding::Encoding;

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
    BadRequest = 400,
    Created = 201,
    InternalServerError = 500,
}

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
            Status::BadRequest => "400 Bad Request",
            Status::Created => "201 Created",
            Status::InternalServerError => "500 Internal Server Error",
        }
    }

    pub fn new(code: isize) -> Option<Status> {
        match code {
            200 => Some(Status::Ok),
            404 => Some(Status::NotFound),
            400 => Some(Status::BadRequest),
            201 => Some(Status::Created),
            500 => Some(Status::InternalServerError),
            _ => None,
        }
    }
}
pub struct Head {
    protocol_version: String,
    method: HTTPMethod,
    status: Status,
    path: String,
    headers: HashMap<String, String>,
}

impl Head {
    pub fn new() -> Head {
        Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            method: HTTPMethod::GET,
            status: Status::Ok,
            path: "".to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn from_string(head_data: &str) -> Result<Head, anyhow::Error> {
        let mut new_head = Head::new();
        let mut lines = head_data.lines();

        new_head.parse_status_line(lines.nth(0))?;

        for line in lines {
            if let Some((key, value)) = line.split_once(":") {
                new_head
                    .headers
                    .insert(key.to_string(), value.trim().to_string());
            }
        }

        Ok(new_head)
    }

    fn parse_status_line(&mut self, line_option: Option<&str>) -> Result<(), anyhow::Error> {
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

    pub fn set_status_line(
        &mut self,
        protocol: String,
        status: Status,
    ) -> Result<(), anyhow::Error> {
        self.protocol_version = protocol;
        self.status = status;
        Ok(())
    }

    pub fn add_header(&mut self, key: String, value: String) -> Result<(), anyhow::Error> {
        self.headers.insert(key, value);
        Ok(())
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn get_path(&self) -> &str {
        &self.path.as_str()
    }

    pub fn get_status_line_string(&self) -> String {
        let mut status_line = self.protocol_version.clone();
        status_line.push(' ');
        status_line.push_str(self.status.to_string());
        status_line.push_str("\r\n");
        status_line
    }

    pub fn get_headers_as_string(&self) -> String {
        let mut headers = String::new();

        for (key, value) in &self.headers {
            headers.push_str(&key);
            headers.push(':');
            headers.push(' ');
            headers.push_str(&value);
            headers.push_str("\r\n");
        }

        headers
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_method(&self) -> &HTTPMethod {
        &self.method
    }

    pub fn get_enconding(&self) -> Option<Encoding> {
        let none = String::from("None");
        let enc_str = self.headers.get("Accept-Encoding").unwrap_or(&none);
        for encoding_str in enc_str.split(',') {
            if let Some(enc) = Encoding::new(encoding_str.trim()) {
                return Some(enc);
            }
        }
        None
    }
}
