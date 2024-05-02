pub mod header;
pub mod linescodec;
pub mod request;

// Uncomment this block to pass the first stage
use crate::{linescodec::LinesCodec, request::Request};
use std::net::{TcpListener, TcpStream};

fn handle_conection(stream: TcpStream) -> anyhow::Result<()> {
    let mut codec = LinesCodec::new(stream)?;
    let req_str = codec.read_message()?;
    print!("Request: {}", req_str);
    let request = Request::from_string(&req_str)?;

    let response = handle_response(request.get_path());
    codec.send_message(&response)?;
    Ok(())
}

fn handle_response(path: &str) -> String {
    let response = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
    };
    return response.to_string();
}

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Couldn't bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                /* let addr = _stream.peer_addr().unwrap(); */
                /* println!("Accepted new connection at {addr:?}"); */

                // let response = b"HTTP/1.1 200 OK\r\n\r\n";
                handle_conection(_stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
