use super::header::{Head, Status, PROTOCOL_VERSION};

pub struct Response {
    head: Head,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        Self {
            head: Head::new(),
            body: "".to_string(),
        }
    }

    pub fn add_status_line(
        &mut self,
        protocol: String,
        status: isize,
    ) -> Result<(), anyhow::Error> {
        self.head
            .set_status_line(protocol, Status::new(status).unwrap())?;
        Ok(())
    }

    pub fn add_header(&mut self, header: String, value: String) -> Result<(), anyhow::Error> {
        self.head.add_header(header, value)?;
        Ok(())
    }

    pub fn add_to_body(&mut self, data: String) {
        self.body.push_str(&data);
    }

    pub fn generate_response_str(&self) -> String {
        let mut response_string = String::new();
        response_string.push_str(self.head.get_status_line_string().as_str());
        response_string.push_str(&self.head.get_headers_as_string().as_str());
        response_string.push_str("\r\n");
        response_string.push_str(&self.body);
        response_string
    }

    pub fn internal_server_error_response() -> Response {
        let mut head = Head::new();
        head.set_status_line(PROTOCOL_VERSION.to_string(), Status::new(500).unwrap())
            .unwrap();
        Self {
            head,
            body: "".to_string(),
        }
    }
}
