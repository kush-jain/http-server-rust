use std::error::Error;
use std::fs;
use std::path::Path;


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

fn handle_user_agent(headers: &Vec<String>) -> Vec<u8> {
    let user_agent = headers.iter().find(|header| header.starts_with("User-Agent: "));
    let response = match user_agent {
        Some(ua) => {
            let ua_value = ua.replace("User-Agent: ", "");
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", ua_value.len(), ua_value)
        },
        None => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
    response.into_bytes()
}

fn handle_file_route(file_path: &String) -> Vec<u8> {

    let directory = std::env::var("APP_DIRECTORY").unwrap_or_else(|_| ".".to_string());

    let path = Path::new(&directory).join(file_path);
    println!("file path: {:?}", path);
    let content = fs::read_to_string(path);

    let response = match content {
        Ok(text) => {
            let length = text.len();
            format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", length, text)
        },
        Err(_e) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
    response.into_bytes()
}


pub fn handle_http_request(request: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {

    if request.is_empty() {
        println!("Empty request");
        return Err("Empty request".into());
    }

    let request_line = &request[0];
    let headers = &request[1..];
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
        r if r.starts_with("/files/") => handle_file_route(&r[7..].to_string()),
        "/user-agent" => handle_user_agent(&headers.to_vec()),
        _ => handle_default(),
    };

    Ok(response)
}


#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use crate::utils;

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
    fn handle_http_request_user_agent_route() {
        let request = vec!["GET /user-agent HTTP/1.1".to_string(), "User-Agent: curl/7.64.1".to_string()];
        let response = handle_http_request(&request).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\ncurl/7.64.1");
    }

    #[test]
    fn handle_http_request_file_route_valid() {
        env::set_var("APP_DIRECTORY", utils::get_project_source().unwrap_or_else(|| ".".to_string()));
        let request = vec!["GET /files/test.txt HTTP/1.1".to_string()];
        let response = handle_http_request(&request).unwrap();
        assert_eq!(
            response,
            b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 13\r\n\r\nHello, World!"
        );
    }

    #[test]
    fn handle_http_request_file_route_invalid() {
        let request = vec!["GET /files/random.txt HTTP/1.1".to_string()];
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
