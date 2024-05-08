pub mod http;
pub mod io_operations;
pub mod routes;
pub mod threadpool;

use crate::{
    http::request::Request, io_operations::linescodec::LinesCodec, routes::handle_response,
    threadpool::Threadpool,
};
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

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Couldn't bind to address");
    let pool = Threadpool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                pool.execute(|| match handle_conection(_stream) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("error: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
