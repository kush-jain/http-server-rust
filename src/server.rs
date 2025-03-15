use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::TcpStream;

use crate::handler;
use crate::interface::{HttpResponse, InternalServerErrorResponse};

pub fn reader(buf_reader: &mut BufReader<&TcpStream>) -> (String, Vec<String>, String) {
    let mut index: i32 = 0;
    let mut request_line = String::new();
    let mut headers = Vec::new();
    let mut request_body = String::new();

    let mut content_length: Option<usize> = None;

    for line in buf_reader.by_ref().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line: {}", e);
                return (request_line, headers, request_body);
            }
        };
        if index == 0 {
            request_line = line;
        } else {
            if line.is_empty() {
                break;
            }
            // Check for Content-Length header
            if let Some(length) = line.strip_prefix("Content-Length: ") {
                content_length = length.parse().ok();
            }
            headers.push(line);
        }

        index += 1;
    }

    // Read the request body if Content-Length header is present
    if let Some(length) = content_length {
        let mut body = vec![0; length];
        if let Err(e) = buf_reader.read_exact(&mut body) {
            println!("Error reading request body: {}", e);
            return (request_line, headers, request_body);
        }
        request_body = match String::from_utf8(body) {
            Ok(body) => body,
            Err(e) => {
                println!("Error converting request body to string: {}", e);
                return (request_line, headers, request_body);
            }
        };
    }

    println!(
        "received request: {:?}, headers: {:?}, request body: {:?}",
        request_line, headers, request_body
    );

    (request_line, headers, request_body)
}

pub fn process_request(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(mut stream) => {
            println!("accepted new connection");

            let mut buf_reader = BufReader::new(&stream);
            let (request_line, headers, request_body) = reader(&mut buf_reader);

            let response =
                match handler::handle_http_request(&request_line, &headers, &request_body) {
                    Ok(response) => response,
                    Err(e) => {
                        println!("Error processing request: {:?}", e);
                        InternalServerErrorResponse.response()
                    }
                };

            if let Err(e) = stream.write(&response) {
                println!("Error writing response: {}", e);
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
