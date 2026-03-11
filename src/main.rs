use std::{
    net::{TcpListener},
};
use web_server::request::handle_request;
use web_server::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_request(stream)
        });
    }

    println!("Shutting down.");
}
