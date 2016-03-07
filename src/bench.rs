#![feature(test)]
use std::io::Read;
use std::env;
use std::fs::File;

extern crate test;

use sfcodec;


#[bench]
fn encode_longe_file(b: &mut test::Bencher) {
    let mut file = File::open("~/text.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text);
    let mut codec = sfcodec::SFCodec::new();
    codec.multithread_with(2);
    b.iter(move || codec.encode(text.clone()));
}
