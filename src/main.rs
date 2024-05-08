pub mod http;
pub mod io_operations;
pub mod routes;
pub mod threadpool;

use io_operations::files::get_arg;

use crate::{
    http::{request::Request, response::Response},
    io_operations::linescodec::LinesCodec,
    routes::handle_response,
    threadpool::Threadpool,
};
use std::net::{TcpListener, TcpStream};

fn handle_conection(stream: TcpStream) -> anyhow::Result<()> {
    let mut codec = LinesCodec::new(stream)?;
    let req_str = codec.read_message()?;
    print!("Request: {}", req_str);
    let request = Request::from_string(&req_str)?;

    let response_result = handle_response(request);
    match response_result {
        Ok(response) => {
            let res_str = response.generate_response_str();
            codec.send_message(&res_str)?;
        }
        Err(_e) => {
            let res_str = Response::internal_server_error_response().generate_response_str();
            codec.send_message(&res_str)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let address = get_arg("--address").unwrap_or_else(|_| "127.0.0.1:4221".to_string());
    let listener = TcpListener::bind(address).expect("Couldn't bind to address");
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
