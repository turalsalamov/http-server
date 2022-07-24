use std::net::{TcpStream, SocketAddr};
use std::io:: {Write, Read };
use crate::server::server_arguments::ServerArgument;

// Accepting the requests
pub fn accepting_incoming(success_argument: ServerArgument) {
    // Infinite loop in order to make server up forever until someione shutdowns it
    loop {
        match success_argument.get_listener().accept() {
            Ok((stream, address)) => handling_the_stream(stream, address),
            Err(_) => error_handling()
        }
    }
    
}


// Handling the incoming stream
fn handling_the_stream(mut stream: TcpStream, address: SocketAddr) {
    let mut buffer = [0; 1024];
    stream.write("Hello World!".as_bytes()).ok().expect("There was an error in writing the message");
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    println!("{}", address.ip());
}


// Error handling for test cases
fn error_handling() {
    println!("There is an error")
}