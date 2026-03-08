use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_request(stream)
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
