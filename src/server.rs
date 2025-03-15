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
        let line = line.unwrap();
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
        buf_reader.read_exact(&mut body).unwrap();
        request_body = String::from_utf8(body).unwrap();
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

            let response = handler::handle_http_request(&request_line, &headers, &request_body)
                .unwrap_or_else(|e| {
                    println!("Error processing request: {:?}", e);
                    InternalServerErrorResponse.response()
                });

            stream.write(&response).unwrap();
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
