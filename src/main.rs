use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::error::Error;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                process_request(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn process_request(mut stream: TcpStream) {

    println!("accepted new connection");

    let buf_reader = BufReader::new(&mut stream);

    // Vector is Rust dynamic growable array. _ means Rust will infer the type.
    let http_request: Vec<_> = buf_reader
        .lines()                                                // Get an iterator over lines. Returns Result<String, std::io::Error>
        .map(|result| result.unwrap_or_else(|_| String::new())) // Unwrap the result or return an empty string
        .take_while(|line| !line.is_empty())                    // HTTP Request ends with an empty line for version 1.1, so return as soon as you get it
        .collect();                                             // Collect the iterator into a vector. This is just for easier processing - we can iterate over the lines directly too.

    let response = handle_http_request(&http_request)
        .unwrap_or_else(|e| {
            println!("Error processing request: {:?}", e);
            b"HTTP/1.1 500 Internal Server Error\r\n\r\n"
        });

    stream.write(response).unwrap();
}


fn handle_http_request(request: &[String]) -> Result<&[u8], Box<dyn Error>> {

    if request.is_empty() {
        println!("Empty request");
        return Err("Empty request".into());
    }

    let request_line = &request[0];
    println!("received request: {:?}", request_line);

    let request_components: Vec<&str> = request_line.split_whitespace().collect();

    if request_components.len() != 3 {
        println!("Invalid request line: {:?}", request_line);
        return Err("Invalid request line".into());
    }

    let route = request_components[1];

    let response: &[u8] = match route {
        "/" => b"HTTP/1.1 200 OK\r\n\r\n",
        _ => b"HTTP/1.1 404 Not Found\r\n\r\n",
    };

    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_http_request_valid_route() {
        let request = vec!["GET / HTTP/1.1".to_string()];
        let response = handle_http_request(&request).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\n\r\n");
    }

    #[test]
    fn handle_http_request_invalid_route() {
        let request = vec!["GET /foo HTTP/1.1".to_string()];
        let response = handle_http_request(&request).unwrap();
        assert_eq!(response, b"HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn handle_http_request_empty_request() {
        let request = vec![];
        let response = handle_http_request(&request);
        assert!(response.is_err());
    }

    #[test]
    fn handle_http_request_invalid_request() {
        let request = vec!["GET".to_string()];
        let response = handle_http_request(&request);
        assert!(response.is_err());
    }
}
