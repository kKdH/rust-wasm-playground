use wasm_sandbox::greet;
use log::{info, Level};

fn main() {
    console_log::init_with_level(Level::Debug);

    info!("Initializing");

    greet();
}
