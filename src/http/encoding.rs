pub enum Encoding {
    Gzip,
}

impl Encoding {
    pub fn new(enconding: &str) -> Option<Encoding> {
        match enconding {
            "gzip" => Some(Encoding::Gzip),
            _ => None,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Encoding::Gzip => String::from("gzip"),
        }
    }
}
