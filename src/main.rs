use std::fs::File;
use std::env;
mod sfcodec;
mod sfsym;

fn main() {
    let mut input = env::args().nth(1).unwrap();
    let mut codec = sfcodec::new();

    codec.encode(input);

    println!("codec: {:?}", codec);
}
