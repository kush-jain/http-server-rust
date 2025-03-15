use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::interface::{self, HttpResponse, InternalServerErrorResponse, NotFoundResponse, OKResponse};


fn handle_root() -> Vec<u8> {
    b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
}

fn handle_default() -> Vec<u8> {
    NotFoundResponse.response()
}

fn handle_echo(content: &String) -> Vec<u8> {
    OKResponse::new(content).response()
}

fn handle_user_agent(headers: &Vec<String>) -> Vec<u8> {
    let user_agent = headers.iter().find(|header| header.starts_with("User-Agent: "));
    let response = match user_agent {
        Some(ua) => {
            let ua_value = ua.replace("User-Agent: ", "");
            OKResponse::new(&ua_value).response()
        },
        None => NotFoundResponse.response(),
    };
    response
}


fn handle_read_file(file_path: &PathBuf) -> Vec<u8> {

    let content = fs::read_to_string(file_path);

    let response = match content {
        Ok(text) => {
            OKResponse::new(text).content_type("application/octet-stream").response()
        },
        Err(_e) => NotFoundResponse.response(),
    };
    response
}


fn handle_write_file(file_path: &PathBuf, content: &String) -> Vec<u8> {

    println!("Writing to file: {:?} contents: {:?}", file_path, content);
    let write_response = fs::write(file_path, content);

    let response = match write_response {
        Ok(_) => interface::OKCreatedResponse.response(),
        Err(_e) => InternalServerErrorResponse.response(),
    };
    response
}


fn handle_file_route(file_path: &String, method: &str, request_body: &String) -> Vec<u8> {

    let directory = std::env::var("APP_DIRECTORY").unwrap_or_else(|_| ".".to_string());

    let path = Path::new(&directory).join(file_path);
    println!("file path: {:?}", path);

    let response = match method.to_uppercase().as_str() {
        "GET" => handle_read_file(&path),
        "POST" => handle_write_file(&path, request_body),
        _ => interface::MethodNotAllowedResponse.response(),
    };
    response
}


pub fn handle_http_request(
    request_line: &String, headers: &Vec<String>, request_body: &String
) -> Result<Vec<u8>, Box<dyn Error>> {

    println!("received request: {:?}, headers: {:?}, request body: {:?}", request_line, headers, request_body);

    let request_components: Vec<&str> = request_line.split_whitespace().collect();

    if request_components.len() != 3 {
        println!("Invalid request line: {:?}", request_line);
        return Err("Invalid request line".into());
    }

    let method = request_components[0];
    let route = request_components[1];

    println!("method: {:?}, route: {:?}", method, route);

    let response = match route {
        "/" => handle_root(),
        r if r.starts_with("/echo/") => handle_echo(&r[6..].to_string()),
        r if r.starts_with("/files/") => handle_file_route(&r[7..].to_string(), method, &request_body),
        "/user-agent" => handle_user_agent(&headers),
        _ => handle_default(),
    };

    Ok(response)
}


#[cfg(test)]
mod tests {
    use std::{env, vec};
    use super::*;
    use crate::utils;

    fn get_inputs(method: &str, route: &str, headers: Option<Vec<&str>>, body: Option<&str>) -> (String, Vec<String>, String) {
        let request_line = format!("{} {} HTTP/1.1", method, route);
        let headers = headers.unwrap_or_else(|| vec![]);
        let body = body.unwrap_or_else(|| "");
        (request_line, headers.iter().map(|h| h.to_string()).collect(), body.to_string())
    }

    #[test]
    fn handle_http_request_valid_route() {
        let (request, headers, body) = get_inputs("GET", "/", None, None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\n\r\n");
    }

    #[test]
    fn handle_http_request_invalid_route() {
        let (request, headers, body) = get_inputs("GET", "/foo", None, None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn handle_http_request_echo_route() {
        let (request, headers, body) = get_inputs("GET", "/echo/foo", None, None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nfoo");
    }

    #[test]
    fn handle_http_request_user_agent_route() {
        let (request, headers, body) = get_inputs(
            "GET", "/user-agent",
            Some(vec!["User-Agent: curl/7.64.1"]),
            None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\ncurl/7.64.1");
    }

    #[test]
    fn handle_http_request_get_file_route_valid() {
        env::set_var("APP_DIRECTORY", utils::get_project_source().unwrap_or_else(|| ".".to_string()));
        let (request, headers, body) = get_inputs("GET", "/files/test.txt", None, None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(
            response,
            b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 13\r\n\r\nHello, World!"
        );
    }

    #[test]
    fn handle_http_request_get_file_route_invalid() {
        let (request, headers, body) = get_inputs("GET", "/files/random.txt", None, None);
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn handle_http_request_post_file_route() {
        env::set_var("APP_DIRECTORY", utils::get_project_source().unwrap_or_else(|| ".".to_string()));
        let (request, headers, body) = get_inputs(
            "POST",
            "/files/abc.txt",
            vec!["Content-Length: 5"].into(),
            "abcde".into());
        let response = handle_http_request(&request, &headers, &body).unwrap();
        assert_eq!(response, b"HTTP/1.1 201 Created\r\n\r\n");

        // Verify the file was created
        let file_path = Path::new(&utils::get_project_source().unwrap_or_else(|| ".".to_string())).join("abc.txt");
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "abcde");

        // Cleanup
        fs::remove_file(&file_path).expect("Cleanup failed");
    }

    #[test]
    fn handle_http_request_invalid_request() {
        let (_request, headers, body) = get_inputs("GET", "/", None, None);
        let request: String = String::from("GET");
        let response = handle_http_request(&request, &headers, &body);
        assert!(response.is_err());
    }
}
