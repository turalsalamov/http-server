use std::io::{ stdin, stdout, Write };
use std::net::{ TcpListener };
use crate::server::server_arguments::{ ServerArgument };


pub fn server_initiator() -> ServerArgument {
    // Getting a desired port number from user
    print!("Please specify the port number: ");
    stdout().flush().ok().expect("There is a problem in flush!"); // flushing the stdout
    let mut port: String = String::new();
    stdin().read_line(&mut port)
                                    .ok()
                                    .expect("There is an error in input!");
    
    // Constructing the ip and port together
    let mut ip_and_port:String = String::from("127.0.0.1");
    ip_and_port.push(':');
    ip_and_port.push_str(port.trim());

    // Binding the listener to specified port for ip
    let listener: TcpListener = match TcpListener::bind(ip_and_port.as_str()){
        Ok(_listener) => _listener,
        Err(_) => panic!("There is a problem in binding the port!")
    };
    
    let server_argument = ServerArgument::new(ip_and_port, listener);
    
    return server_argument;
}