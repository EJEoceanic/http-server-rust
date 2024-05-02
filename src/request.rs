use anyhow::Ok;

use crate::header::Header;

pub struct Request {
    header: Header,
    data: String,
}

impl Request {
    pub fn new() -> Request {
        Self {
            header: Header::new(),
            data: String::new(),
        }
    }

    pub fn from_string(request: &str) -> Result<Request, anyhow::Error> {
        let mut new_request = Request::new();
        let (header_data, data) = request.split_once("\r\n\r\n").unwrap_or_default();

        println!("Meow 1: {} Meow 2: {}", header_data, data);

        new_request.header = Header::from_string(header_data)?;
        new_request.data = data.to_string();
        Ok(new_request)
    }

    pub fn get_path(&self) -> &str {
        self.header.get_path()
    }
}
