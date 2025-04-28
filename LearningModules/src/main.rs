mod riftgate_network;
mod utils;

use riftgate_network::{client::data_layer::send_data, server::ports::listen};
use utils::{helper_function, some_public_function};

fn main() {
    println!("Starting main application");
    some_public_function();
    helper_function();
    listen();

    riftgate_network::connect();
    send_data();

    println!("Finished main application");
}
