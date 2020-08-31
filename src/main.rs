use std::env;

pub mod utils;

fn main() {
    // utils::open_in_bytes("hello-arm"); // filename
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    utils::open_in_bytes(filename);
}
