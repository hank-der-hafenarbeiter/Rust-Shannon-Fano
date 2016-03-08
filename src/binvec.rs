use std::boxed;
use utils;

pub struct BinVec {
    content:Vec<u8>,
    in_byte_pos:usize, //gives the first free position in the last byte of the vector
    appended_vector:Option<Box<BinVec>>,
}

impl BinVec {

    pub fn new () -> BinVec {
        BinVec{ content:Vec::new(), in_byte_pos:0, appended_vector:None}
    }

    pub fn push(&mut self, value:bool) {
        if self.in_byte_pos == 0 {
            if value {
                self.content.push(1);
            } else {
                self.content.push(0);
            }
        } else if value {
            let bit_mask:u8 = 0b1000_0000 >> self.in_byte_pos;
            let mut byte = self.content.pop().unwrap(); //TODO: error handling
            byte = bit_mask | byte;
            self.content.push(byte);
        }
        self.in_byte_pos += 1;
    }

    pub fn append_bin_vec(&mut self, other:BinVec) {
        if let None = self.appended_vector {
            self.appended_vector = Some(Box::new(other));
        } else {
            self.appended_vector.as_mut().unwrap().append_bin_vec(other);
        }
    }

    pub fn append_byte(&mut self, values:[bool;8]) {
        if self.in_byte_pos == 0 {                       //fresh byte can be used
            self.content.push(utils::bools_to_byte(values));
        } else {
            let mut byte = self.content.pop().unwrap();     //TODO: error handling
            let mut bit_mask:u8 = 0b00000001;
            bit_mask << self.in_byte_pos +1;

            for i in 0..8-self.in_byte_pos {
                if values[i] {
                   byte = (byte & bit_mask);
                }
                bit_mask << 1;
            } 
            self.content.push(byte);

            byte = 0;
            bit_mask = 0b00000001; 
            for i in 8-self.in_byte_pos..8 {
                if values[i] {
                    byte = (byte & bit_mask);
                }
                bit_mask << 1;
            }
            byte << (8-self.in_byte_pos);

            self.content.push(byte);

        }
    }

    pub fn as_bytes<'a>(&'a self) -> BinVecBytewise<'a> {
        BinVecBytewise::new(self,0)
    }
}

struct BinVecBytewise<'a> {
    cur_vector:&'a BinVec,
    cur_pos:usize,
}
impl<'a> BinVecBytewise<'a> {

    fn new(vector:&'a BinVec, pos:usize) -> BinVecBytewise {
        BinVecBytewise{cur_vector:vector, cur_pos:pos}
    }
}

impl<'a> Iterator for BinVecBytewise<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.cur_pos+1 < self.cur_vector.content.len() {     //next step still in current vector
            self.cur_pos += 1;
            return Some(self.cur_vector.content[self.cur_pos]);
        } else {
            let result = &self.cur_vector.appended_vector.and_then(|nextVec| nextVec.as_bytes().next());
            result
        }
    }
}
