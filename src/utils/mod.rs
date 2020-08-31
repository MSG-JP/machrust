use std::fs::File;
use std::io::Read;

pub fn open_in_bytes(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read");
    buf
}
