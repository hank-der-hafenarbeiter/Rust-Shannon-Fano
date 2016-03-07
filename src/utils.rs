extern crate rand;


/// Mask of the value bits of a continuation byte
const CONT_MASK: u8 = 0b0011_1111;
/// Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte
const TAG_CONT_U8: u8 = 0b1000_0000;


 fn is_acceptable_index(text:&String, index: usize) -> bool {
    if index == text.len() {
        true
    } else {
        text.as_bytes().get(index).map_or(false, |byte| {
            // check it's not a
            // continuation byte
            (byte & !CONT_MASK) != TAG_CONT_U8
        })
    }
}

pub fn find_previous_char_boundary(text:&String, index:usize) -> usize {
    let mut index = index;
    while !is_acceptable_index(text, index) {
        index -=1;
    }
    index
}

pub fn bools_to_byte(bools:[bool;8]) -> u8 {
    let mut result:u8 = 0b0000_0000;
    let mut bit_mask = 0b1000_0000;
    for i in 0..8 {
        println!("bit_mask = {:08b}", bit_mask);
        if bools[i] {
            result = result | bit_mask;
        }
        bit_mask =  bit_mask >> 1;
    }
    result
}

pub fn byte_to_bools(byte:u8) -> [bool;8] {
    let mut result = [false;8];
    let mut bit_mask = 0b1000_0000;

    for i in 0..8 {
        result[i] = (bit_mask | byte) != 0;
        bit_mask = bit_mask >> 1;
    }
    result
}

#[test]
fn test_bool_to_byte() {
    extern crate rand;
    use self::rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    for i in 0..20 {
        let mut bools = [false;8];
        let mut num:u8 = 0;
        for i in 0..8 {
            bools[i] = rng.gen();
            if bools[i] {num = num | (0b1000_0000 >>  i);}
        }
        println!("{}: {:08b} ==  {:08b}",i, num, bools_to_byte(bools));
        assert!(num == bools_to_byte(bools));
    }
}

#[test]
fn test_byte_and_back() {
    extern crate rand;
    use self::rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    for i in 0..20 {
        let mut num:u8 = 0;
        for i in 0..8 {
            num = rng.gen();
        }
        
        assert!(num, bools_to_byte(byte_to_bools(num)));
    }
}
 
 
 
