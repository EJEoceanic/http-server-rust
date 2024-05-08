use std::io::{self, BufRead, BufReader, LineWriter, Result, Write};
use std::net::TcpStream;

pub struct LinesCodec {
    reader: BufReader<TcpStream>,
    writer: LineWriter<TcpStream>,
}

impl LinesCodec {
    pub fn new(stream: TcpStream) -> Result<LinesCodec> {
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let recieved: Vec<u8> = self.reader.fill_buf()?.to_vec();
        self.reader.consume(recieved.len());
        Ok(String::from_utf8_lossy(&recieved).to_string())
    }
}
