use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

// Entry point of the program
fn main() {
    // Create a TCP listener bound to the local address 127.0.0.1 on port 7878
    // Port 7878 is used because it spells "rust" on a phone keypad
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // Continuously listen for incoming TCP connections
    for stream in listener.incoming() {
        // If a connection is successfully established, handle the connection
        if let Ok(stream) = stream {
            handle_connection(stream);
        }
    }
}

// Function to handle an individual TCP connection
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to store data read from the TCP stream
    let mut buffer = [0; 1024];
    
    // Read data from the TCP stream into the buffer
    if let Ok(_) = stream.read(&mut buffer) {
        // Determine the HTTP status line and the filename to serve based on the request
        let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            // If the request is a GET request for the root path, serve "index.html"
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            // For any other request, serve "404.html" with a 404 Not Found status
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        // Read the contents of the specified file into a string
        let contents = fs::read_to_string(filename).unwrap_or_default();
        
        // Create the full HTTP response by formatting the status line, headers, and body
        let response = format!(
            "{}\r\n\
            Content-Type: text/html; charset=UTF-8\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            status_line,
            contents.len(),
            contents
        );
        
        // Write the HTTP response to the TCP stream
        if let Err(_) = stream.write_all(response.as_bytes()) {
            // If writing to the stream fails, print an error message to stderr
            eprintln!("Failed to send response");
        }
    }
}
