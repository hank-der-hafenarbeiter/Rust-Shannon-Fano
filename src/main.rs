use std::io::Read;
use std::env;
use std::fs::File;

mod sfcodec;
mod sfsym;
mod sfvec;
mod binvec;
mod utils;


fn main() {
    let input = env::args().nth(1).unwrap();
    let mut file = File::open(input).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text);

    let mut codec = sfcodec::SFCodec::new();
    codec.multithread_with(32);
    let text = text.clone();
    codec.encode(text);
    
}
