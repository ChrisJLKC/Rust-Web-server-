use Multi::ThreadPool; // search all files in the repository for the function Threadpool.
use std::fs; // allows rust to locate files in the respository.
use std::io::prelude::*; // library used for importing on and off traits 
use std::net::TcpListener; // library used for listening to responses of local TCP connection
use std::net::TcpStream; // library used for helping rust to respond to the responses of the local TCP connection
use std::thread; // library used to control the amount of threads used on a server 
use std::time::Duration; // library that imports time into rust, to allow wait intervals 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // allows the TcpListener library read responses and add replies to the responses on that local TCP socket
    let pool = ThreadPool::new(4); // creates a new Threadpool of 4 threads, make sure that your device can handle this amount of threads

    for stream in listener.incoming().take(2) { // listens for a local device to enter into the local socket and takes two threads to use to speed up the process 
        let stream = stream.unwrap(); // allows stream to return the local device if local device is there and produce the content

        pool.execute(|| { // uses the pool amount created above to execute the file with the intended amount of threads 
            handle_connection(stream); // makes sure there is a connection with the local device 
        });
    }

    println!("Shutting down."); // In terminal, shutting down local socket server 
}

fn handle_connection(mut stream: TcpStream) { // borrows data of local device, produces content to the local device 
    let mut buffer = [0; 1024]; // makes the borrowed data to be only between 0 and 1024 bytes big in memory
    stream.read(&mut buffer).unwrap(); // uses the TcpStream library to read the address of the borrowed buffer and displays it to the device

    let get = b"GET / HTTP/1.1\r\n"; // makes get variable to find out that the file name required, is a html file
    let sleep = b"GET /sleep HTTP/1.1\r\n"; // creates a variable called sleep for the interval of wait time 

    let (status_line, filename) = if buffer.starts_with(get) { // makes two variables for the status of the connection and the file, that goes with the buffer of the get variable
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") // uses 200 bytes of the buffer, makes sure that file starts with HTTP, and then file name
    } else if buffer.starts_with(sleep) { // if user has placed sleep in the address bar after the socket and port like "/sleep" it will do the following
        thread::sleep(Duration::from_secs(5)); // the server waits 5 seconds before loading the website, simulating a slow request
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") // uses 200 bytes of the buffer, makes sure that file starts with HTTP, and then file name
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html") // if file not found, will show this file
    };

    let contents = fs::read_to_string(filename).unwrap(); // uses library fs to read the file and then shows it to the user 

    let response = format!("{}{}", status_line, contents); // then it creats a variable called response which formats for a response to the TCP request of local socket the status of the file type and the contents of the file

    stream.write(response.as_bytes()).unwrap(); // writes the reponse to the TCP request using the response variable 
    stream.flush().unwrap(); // when server is shut down, it shuts it off quickly and efficiently
}


/* 1. Simulating a slow request using the library thread

use std::fs; 
use std::io::prelude::*; 
use std::net::TcpListener; 
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() { 
        let stream = stream.unwrap(); 

        thread::spawn(|| {
            handle_connection(stream); 
        });
    }
}

fn handle_connection(mut stream: TcpStream) { 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); 

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";  
    
    let (status_line, _filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") 
    }

    else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html") 
    }

    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();  
        stream.flush().unwrap();
    
} */
