use crate::header::Head;

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
}
