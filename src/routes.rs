use crate::{
    files::{read_file, write_file},
    header::{HTTPMethod, PROTOCOL_VERSION},
    response::Response,
    Request,
};

pub fn handle_response(request: Request) -> Result<Response, anyhow::Error> {
    let path = request.get_path_as_vec();

    let mut response = Response::new();

    match path[0] {
        "" => {
            handle_empty_path(&mut response)?;
        }

        "echo" => {
            handle_echo_path(&mut response, request)?;
        }
        "user-agent" => {
            handle_user_agent_path(&mut response, request)?;
        }
        "files" => {
            handle_files_path(&mut response, request)?;
        }
        _ => {
            response.add_status_line(PROTOCOL_VERSION.to_string(), 404)?;
        }
    }
    Ok(response)
}

fn handle_empty_path(response: &mut Response) -> Result<&Response, anyhow::Error> {
    response.add_status_line(PROTOCOL_VERSION.to_string(), 200)?;
    Ok(response)
}

fn handle_echo_path(response: &mut Response, request: Request) -> Result<&Response, anyhow::Error> {
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

fn handle_user_agent_path(
    response: &mut Response,
    request: Request,
) -> Result<&Response, anyhow::Error> {
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

fn handle_files_path<'a>(
    response: &'a mut Response,
    request: Request,
) -> Result<&'a Response, anyhow::Error> {
    let path = request.get_path_as_vec();
    let filename = path.get(1);

    match request.get_method() {
        HTTPMethod::GET => {
            match filename {
                Some(file_path) => {
                    let file_read = read_file(&file_path);
                    match file_read {
                        Ok(file_content) => {
                            response.add_status_line(PROTOCOL_VERSION.to_string(), 200)?;
                            response.add_header(
                                String::from("Content-Type"),
                                String::from("application/octet-stream"),
                            )?;
                            response.add_header(
                                String::from("Content-Length"),
                                file_content.len().to_string(),
                            )?;
                            response.add_to_body(file_content);
                        }
                        Err(_e) => {
                            response.add_status_line(PROTOCOL_VERSION.to_string(), 404)?;
                        }
                    }
                }
                None => {
                    // No path provided
                    response.add_status_line(PROTOCOL_VERSION.to_string(), 400)?;
                    response.add_to_body(String::from("No path was provided"));
                }
            };
        }
        HTTPMethod::POST => match filename {
            Some(file_name) => {
                let body = request.get_body();
                let file_write_res = write_file(&file_name, &body);

                match file_write_res {
                    Ok(()) => {
                        response.add_status_line(PROTOCOL_VERSION.to_string(), 201)?;
                        response
                            .add_header(String::from("Content-Type"), String::from("text/plain"))?;
                        response
                            .add_header(String::from("Content-Length"), body.len().to_string())?;
                        response.add_to_body(body.to_string());
                    }
                    Err(_) => {
                        response.add_status_line(PROTOCOL_VERSION.to_string(), 500)?;
                        response.add_to_body("Error writting the file".to_string());
                    }
                }
            }
            None => {
                response.add_status_line(PROTOCOL_VERSION.to_string(), 400)?;
                response.add_to_body(String::from("No path was provided"));
            }
        },
        _ => {}
    };

    Ok(response)
}
