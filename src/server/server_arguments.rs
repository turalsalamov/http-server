use std::net::TcpListener;

pub struct ServerArgument {
    ip_and_port: String,
    listener: TcpListener
}

impl ServerArgument {

    pub fn new(address: String, listener_argument: TcpListener) -> Self {
        Self {ip_and_port: address, listener: listener_argument}
    }

    pub fn get_ip_and_port(&self) -> &String {
        &self.ip_and_port
    }
    pub fn get_listener(&self) -> &TcpListener {
        &self.listener
    }
}

