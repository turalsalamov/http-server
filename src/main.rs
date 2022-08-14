mod server;
mod environment;


// use std::env;

use crate::environment::work_with_env;
// use futures::executor::block_on;
// use crate::server::server_initiator;
// use crate::server::incoming_handler;



fn main() {


    // Environment variable handling
    work_with_env::app_env();
    // Now we can get the env variables by using std::env library and use it for our purpose



    // Initiating the server with the user port input
    // let success_argument = server_initiator::server_initiator();
    // Accepting and handling the incoming requests
    // incoming_handler::accepting_incoming(success_argument)
    
}
