use std::fs; // library used for allowing rust to use a file in the code
use std::io::prelude::*; // library used for importing on and off traits 
use std::net::TcpListener; // library used for listening to responses of local TCP connection
use std::net::TcpStream; // library used for helping rust to respond to the responses of the local TCP connection

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // allows the TcpListener library read responses and add replies to the responses on that local TCP socket

    for stream in listener.incoming() { // listens for a local device to enter into the local socket
        let stream = stream.unwrap(); // allows stream to return the local device if local device is there and produce the content

        handle_connection(stream); // makes sure there is a connection with the local device 
    }
}

fn handle_connection(mut stream: TcpStream) { // borrows data of local device, produces content to the local device 
    let mut buffer = [0; 1024]; // makes the borrowed data to be only between 0 and 1024 bytes big in memory 
    stream.read(&mut buffer).unwrap(); // uses the TcpStream library to read the address of the borrowed buffer and displays it to the device

    let get = b"GET / HTTP/1.1\r\n"; // makes get variable to find out that the file name required, is a html file
    
    let (status_line, _filename) = if buffer.starts_with(get) { // makes two variables for the status of the connection and the file, that goes with the buffer of the get variable
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") // uses 200 bytes of the buffer, makes sure that file starts with HTTP, and then file name
    }

    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html") // if file not found, will show this file
    };
    
        let contents = fs::read_to_string("index.html").unwrap(); // uses library fs to read the file and then shows it to the user 

        let response = format!("{}{}", status_line, contents); // then it creats a variable called response which formats for a response to the TCP request of local socket the status of the file type and the contents of the file

        stream.write(response.as_bytes()).unwrap(); // writes the reponse to the TCP request using the response variable 
        stream.flush().unwrap(); // when server is shut down, it shuts it off quickly and efficiently 
    
}


// 5. "Using conditional statements to determine if the right webpage is being uploaded"

//use std::fs;
//use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;

//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//    for stream in listener.incoming() {
//        let stream = stream.unwrap();

//        handle_connection(stream);
//    }
//}

//fn handle_connection(mut stream: TcpStream) {
//    let mut buffer = [0; 1024];
//    stream.read(&mut buffer).unwrap();

//    let get = b"GET / HTTP/1.1\r\n";
    
//    if buffer.starts_with(get) {
//        let contents = fs::read_to_string("index.html").unwrap();

//        let response = format!(
//            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//            contents.len(),
//            contents
//        );

//        stream.write(response.as_bytes()).unwrap();
//        stream.flush().unwrap();
//    }

//    else {
        //other request
//    }
//}


// 4. "Locating a html file and using it as the returning response"

//use std::fs;
//use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;

//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//    for stream in listener.incoming() {
//        let stream = stream.unwrap();

//        handle_connection(stream);
//    }
//}

//fn handle_connection(mut stream: TcpStream) {
//    let mut buffer = [0; 1024];

//    stream.read(&mut buffer).unwrap();

//    let contents = fs::read_to_string("index.html").unwrap();

//    let response = format!(
//        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//        contents.len(),
//        contents
//    );

//    stream.write(response.as_bytes()).unwrap();
//    stream.flush().unwrap();
//}


// 3. "Writing a Response to the returning response"

//use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;

//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//    for stream in listener.incoming() {
//        let stream = stream.unwrap();

//        handle_connection(stream);
//    }
//}

//fn handle_connection(mut stream: TcpStream) {
//    let mut buffer = [0; 1024];

//    stream.read(&mut buffer).unwrap();

//    let response = "HTTP/1.1 200 OK\r\n\r\n";

//    stream.write(response.as_bytes()).unwrap();
//    stream.flush().unwrap();
//}


// 2. "Returning a Response"


//use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;

//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//    for stream in listener.incoming() {
//        let stream = stream.unwrap();

//        handle_connection(stream);
//    }
//}

//fn handle_connection(mut stream: TcpStream) {
//    let mut buffer = [0; 1024];

//    stream.read(&mut buffer).unwrap();

//    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//}


// 1. "Establishing a connection"

//use std::net::TcpListener;

//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//
//    for stream in listener.incoming() {
//        let stream = stream.unwrap();
//
//        println!("Connection established!");
//    }
//}

