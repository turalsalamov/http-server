use std::io::{ stdin, stdout, Write };
use std::net::{ TcpListener };
fn main() {

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
    let listener = TcpListener::bind(ip_and_port.as_str())
                                            .ok()
                                            .expect("There is a problem in binding the ip and port!");
    // Accepting the requests
    loop {
        match listener.accept() {
            Ok((_socket, _addr)) => println!("Everything is ok"),
            Err(_e) => println!("There is an error in accepting the requests"),
        }
    }

}
