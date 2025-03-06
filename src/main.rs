use std::io::{Write, BufReader, BufRead, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;

use clap::Parser;

mod handler;
mod utils;


#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    directory: Option<String>,
}


fn main() {

    let args = Args::parse();

    // Set environment variables based on command-line args
    if let Some(directory) = args.directory {
        env::set_var("APP_DIRECTORY", directory.to_string());
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        // Not Recommended for production - use Threadpool instead
        // https://doc.rust-lang.org/stable/book/ch21-02-multithreaded.html
        thread::spawn(move || {
            process_request(stream);
        });
    }
}

fn process_request(stream: Result<TcpStream, Error>) {

    match stream {
        Ok(mut stream) => {
            println!("accepted new connection");

            let buf_reader = BufReader::new(&mut stream);

            // Vector is Rust dynamic growable array. _ means Rust will infer the type.
            let http_request: Vec<_> = buf_reader
                .lines()                                                // Get an iterator over lines. Returns Result<String, std::io::Error>
                .map(|result| result.unwrap_or_else(|_| String::new())) // Unwrap the result or return an empty string
                .take_while(|line| !line.is_empty())                    // HTTP Request ends with an empty line for version 1.1, so return as soon as you get it
                .collect();                                             // Collect the iterator into a vector. This is just for easier processing - we can iterate over the lines directly too.

            let response = handler::handle_http_request(&http_request)
                .unwrap_or_else(|e| {
                    println!("Error processing request: {:?}", e);
                    b"HTTP/1.1 500 Internal Server Error\r\n\r\n".to_vec()
                });

            stream.write(&response).unwrap();
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
