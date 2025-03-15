use std::collections::HashMap;
use std::any::Any;

use crate::utils::gzip_compress;


# [derive(Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn with_content_type<B: Into<String>>(mut self, content_type: B) -> Self {
        self.headers.insert("Content-Type".to_string(), content_type.into());
        self
    }

    pub fn with_content_length<B: Into<String>>(mut self, content_length: B) -> Self {
        self.headers.insert("Content-Length".to_string(), content_length.into());
        self
    }

    pub fn with_encoding<B: Into<String>>(mut self, encoding: B) -> Self {
        self.headers.insert("Content-Encoding".to_string(), encoding.into());
        self
    }

    pub fn to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}


pub trait HttpResponse: Any {
    fn response(&self) -> Vec<u8>;

    fn as_any(&self) -> &dyn Any;
}

# [derive(Clone)]
pub struct OKResponse {
    headers: HttpHeaders,
    body: Vec<u8>,
}

impl HttpResponse for OKResponse {
    fn response(&self) -> Vec<u8> {
        let content_length = self.body.len();
        let self_headers = self.headers.clone();
        let headers = self_headers.with_content_length(content_length.to_string());
        let mut response = format!(
            "HTTP/1.1 200 OK\r\n{}\r\n\r\n",
            headers.to_string(),
        ).into_bytes();

        response.extend(&self.body);
        response
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OKResponse {
    pub fn new<B: Into<String>>(body: B) -> Self {
        Self {
            headers: HttpHeaders::new().with_content_type("text/plain"),
            body: body.into().into_bytes(),
        }
    }

    pub fn compress(mut self) -> Self {
        self.headers = self.headers.with_encoding("gzip");
        self.body = gzip_compress(&self.body);
        self
    }

    pub fn with_content_type<H: Into<String>>(mut self, content_type: H) -> Self {
        self.headers = self.headers.with_content_type(content_type);
        self
    }
}

pub struct OKCreatedResponse;

impl HttpResponse for OKCreatedResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 201 Created\r\n\r\n".to_string().into_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NotFoundResponse;

impl HttpResponse for NotFoundResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string().into_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForbiddenResponse;

impl HttpResponse for ForbiddenResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 403 Forbidden\r\n\r\n".to_string().into_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MethodNotAllowedResponse;

impl HttpResponse for MethodNotAllowedResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 405 Method Not Allowed\r\n\r\n"
            .to_string()
            .into_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InternalServerErrorResponse;

impl HttpResponse for InternalServerErrorResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 500 Internal Server Error\r\n\r\n"
            .to_string()
            .into_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
