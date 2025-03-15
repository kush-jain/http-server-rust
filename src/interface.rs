
pub trait HttpResponse {
    fn response(&self) -> Vec<u8>;
}


pub struct OKResponse {
    content_type: String,
    body: String
}

impl HttpResponse for OKResponse {

    fn response(&self) -> Vec<u8> {
        let content_length = self.body.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", self.content_type, content_length, self.body);
        response.into_bytes()
    }
}

impl OKResponse {
    pub fn new<B: Into<String>>(
        body: B
    ) -> Self {
        Self {
            content_type: "text/plain".to_string(),
            body: body.into()
        }
    }

    pub fn content_type<B: Into<String>>(
        mut self,
        content_type: B
    ) -> Self {
        self.content_type = content_type.into();
        self
    }
}


pub struct OKCreatedResponse;

impl HttpResponse for OKCreatedResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 201 Created\r\n\r\n".to_string().into_bytes()
    }
}


pub struct NotFoundResponse;

impl HttpResponse for NotFoundResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string().into_bytes()
    }
}


pub struct MethodNotAllowedResponse;

impl HttpResponse for MethodNotAllowedResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string().into_bytes()
    }
}


pub struct InternalServerErrorResponse;

impl HttpResponse for InternalServerErrorResponse {
    fn response(&self) -> Vec<u8> {
        "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string().into_bytes()
    }
}
