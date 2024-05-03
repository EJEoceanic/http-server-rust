pub mod header;
pub mod linescodec;
pub mod request;
pub mod response;

use header::PROTOCOL_VERSION;
use response::Response;

// Uncomment this block to pass the first stage
use crate::{linescodec::LinesCodec, request::Request};
use std::net::{TcpListener, TcpStream};

fn handle_conection(stream: TcpStream) -> anyhow::Result<()> {
    let mut codec = LinesCodec::new(stream)?;
    let req_str = codec.read_message()?;
    print!("Request: {}", req_str);
    let request = Request::from_string(&req_str)?;

    let response = handle_response(request)?;

    let res_str = response.generate_response_str();
    codec.send_message(&res_str)?;
    Ok(())
}

fn handle_response(request: Request) -> Result<Response, anyhow::Error> {
    /*     let response = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
    }; */
    let path = request.get_path_as_vec();

    let mut response = Response::new();

    match path[0] {
        "" => {
            response.add_status_line(PROTOCOL_VERSION.to_string(), 200)?;
            Ok(response)
        }
        "echo" => {
            let body = if let Some((_, input)) = request.get_path().split_once("echo/") {
                input
            } else {
                ""
            };
            response.add_status_line(PROTOCOL_VERSION.to_string(), 200)?;
            response.add_header(String::from("Content-Type"), String::from("text/plain"))?;
            response.add_header(
                String::from("Content-Length"),
                String::from(body.len().to_string()),
            )?;
            response.add_to_body(String::from(body));

            Ok(response)
        }
        "user-agent" => {
            let body = if let Some(agent) = request.get_header("User-Agent") {
                agent
            } else {
                ""
            };

            response.add_status_line(PROTOCOL_VERSION.to_string(), 200)?;
            response.add_header(String::from("Content-Type"), String::from("text/plain"))?;
            response.add_header(
                String::from("Content-Length"),
                String::from(body.len().to_string()),
            )?;
            response.add_to_body(String::from(body));
            Ok(response)
        }
        _ => {
            response.add_status_line(PROTOCOL_VERSION.to_string(), 404)?;
            Ok(response)
        }
    }
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
