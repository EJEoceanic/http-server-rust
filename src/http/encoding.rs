use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Copy, Clone)]
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

    pub fn encode(self, content: &str) -> Result<String, anyhow::Error> {
        match self {
            Encoding::Gzip => encode_gzip(content),
        }
    }
}

fn encode_gzip(content: &str) -> Result<String, anyhow::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    match encoder.write_all(content.as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    let compressed_string;
    match encoder.finish() {
        Ok(compressed_bytes) => {
            unsafe {
                compressed_string = String::from_utf8_unchecked(compressed_bytes);
            };
            Ok(compressed_string)
        }
        Err(err) => Err(anyhow::format_err!("{} Error compressing string", err)),
    }
}
