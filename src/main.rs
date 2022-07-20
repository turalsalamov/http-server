use std::io::{ stdin, stdout, Write };
use std::net::{ TcpListener };
fn main() {

    print!("Please specify the port number: ");
    stdout().flush().ok().expect("There is a problem in flush!");
    let mut port: String = String::new();

    stdin().read_line(&mut port)
                                    .ok()
                                    .expect("There is an error in input!");
    
    let mut ip_and_port:String = String::from("127.0.0.1");
    ip_and_port.push(':');
    ip_and_port.push_str(port.as_str());

    let listener = TcpListener::bind(&ip_and_port)
                                    .ok()
                                    .expect("There is a problem in binding the ip and port!");
    match listener.accept() {
        Ok((_socket, _addr)) => println!("Everything is ok"),
        Err(_e) => println!("There is an error in accepting the requests"),
    }

}
