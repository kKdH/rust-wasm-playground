use wasm_sandbox::greet;
use log::{info, Level};

fn main() {
    console_log::init_with_level(Level::Debug)
        .expect("Successful initialization of the global logger.");

    info!("Initializing");

    greet();
}
