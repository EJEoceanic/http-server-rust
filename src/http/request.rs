use itertools::Itertools;

use super::header::{HTTPMethod, Head};

pub struct Request {
    header: Head,
    data: String,
}

impl Request {
    pub fn new() -> Request {
        Self {
            header: Head::new(),
            data: String::new(),
        }
    }

    pub fn from_string(request: &str) -> Result<Request, anyhow::Error> {
        let mut new_request = Request::new();
        let (header_data, data) = request.split_once("\r\n\r\n").unwrap_or_default();

        new_request.header = Head::from_string(header_data)?;
        new_request.data = data.to_string();

        Ok(new_request)
    }

    pub fn get_path(&self) -> &str {
        self.header.get_path()
    }

    pub fn get_path_as_vec(&self) -> Vec<&str> {
        let elements = self
            .header
            .get_path()
            .trim_matches('/')
            .split("/")
            .collect_vec();

        elements
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.header.get_headers().get(key)
    }

    pub fn get_method(&self) -> &HTTPMethod {
        return self.header.get_method();
    }

    pub fn get_body(&self) -> &String {
        return &self.data;
    }

    pub fn get_enconding(&self) -> Option<super::encoding::Encoding> {
        self.header.get_enconding()
    }
}
