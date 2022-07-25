use std::collections::{HashSet, HashMap};
use std::net::{TcpStream, SocketAddr};
use std::io:: {Write, Read, stdout };
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
    let data = String::from_utf8_lossy(&buffer[..]);
    println!("{}", data);
    let vec: Vec<&str> = data.split("\r\n").collect();

    /*
        The fisrt line will identify with which method which path is requested.
        This line is splitted with whitespace splitting and collected to a vector
        The first_line variable is also splitted in order to identify aformentioned fields
    */ 
    let first_line: Vec<&str> = vec[0].split_whitespace().collect();

    /*
        The other lines is splitted with ':' and they will be pushed to HashMap in order to have 
        structered data structure.
    */
    let mut header_hashmap: HashMap<&str, &str> = HashMap::new();
    for i in 1..vec.capacity() - 3 {
        let line: Vec<&str> = vec[i].split(':').collect();
        let key = line[0];
        println!("{}", key);
        // let vec_for_value: Vec<&str> = line[1].split_whitespace().collect();
        // let value = vec_for_value[0];
        // header_hashmap.insert(key, value);

    }


    // for element in first_line {
    //     print!("{} ", element);
    // }
    // println!();


}


// Error handling for test cases
fn error_handling() {
    println!("There is an error")
}