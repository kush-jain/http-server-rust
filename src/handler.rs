use std::error::Error;

fn handle_root() -> Vec<u8> {
    b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
}

fn handle_default() -> Vec<u8> {
    b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
}

fn handle_echo(content: &String) -> Vec<u8> {
    let length = content.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", length, content);
    response.into_bytes()
}


pub fn handle_http_request(request: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {

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

    let response = match route {
        "/" => handle_root(),
        r if r.starts_with("/echo/") => handle_echo(&r[6..].to_string()),
        _ => handle_default(),
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
    fn handle_http_request_echo_route() {
        let request = vec!["GET /echo/foo HTTP/1.1".to_string()];
        let response = handle_http_request(&request).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nfoo");
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
