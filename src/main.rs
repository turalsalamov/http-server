mod server;
use crate::server::server_initiator;
use crate::server::incoming_handler;

fn main() {
    // Initiating the server with the user port input
    let success_argument = server_initiator::server_initiator();
    // Accepting and handling the incoming requests
    incoming_handler::accepting_incoming(success_argument)
    
}
