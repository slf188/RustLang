use std::fs;
// to read and write from the stream
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // listen to tcp connections at specified address
    // 7878 is the port
    // bind will return a new tcplistener instance
    // bind means binding to a port
    // bind returns Result<T, E> i.e. the binding might fail
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // incoming is an iterator that returns a sequence of streams
    // each stream resembles an open connection between client and server
    for stream in listener.incoming() {
        // unwrap will terminate our program if the stream has any errors
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // buffer will hold the data that is read
    let mut buffer = [0; 1024];
    // stream.data will read bytes and put those in the buffer
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();    
}

