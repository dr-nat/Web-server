use std::{
    fs,
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
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 Ok\r\n\r\n";
        let contents = fs::read_to_string("natty.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
