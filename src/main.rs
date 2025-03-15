use std::env;
use std::net::TcpListener;
use std::thread;

use clap::Parser;

mod handler;
mod interface;
mod server;
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
            server::process_request(stream);
        });
    }
}
