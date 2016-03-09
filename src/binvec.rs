use std::boxed;
use std::slice;
use utils;

pub struct BinVec {
    content:Vec<u8>,
    in_byte_pos:usize, //gives the first free position in the last byte of the vector
    appended_vector:Option<Box<BinVec>>,
}

impl BinVec {

    pub fn new () -> BinVec {
        BinVec{ content:vec!(0), in_byte_pos:8, appended_vector:None}
    }

    pub fn push(&mut self, value:bool) {
        if self.in_byte_pos == 8 {
            if value {
                self.content.push(0b1000_0000);
            } else {
                self.content.push(0);
            }
            self.in_byte_pos = 0;   //incremented further down
        } else if value {
            let bit_mask:u8 = 0b1000_0000 >> self.in_byte_pos;
            self.content.last_mut().map(|byte| *byte | bit_mask); 
        }
        self.in_byte_pos += 1;
    }

    pub fn pop(&mut self) -> Option<bool> {
        self.in_byte_pos -= 1;
        if self.in_byte_pos == 0 {
            self.in_byte_pos = 8;
            return self.content.pop().map(|x| x != 0); 
        } else { 
            return self.content.last().map(|x| x & (0b1000_0000 >> self.in_byte_pos) != 0);
        }
    }
   
    //returns a full byte (if the last byte is full used) or an error
    pub fn pop_byte(&mut self) -> Result<u8, &str> {
        if self.in_byte_pos == 7 {
            return self.content.pop().ok_or("ERROR: Last byte not fully in use! Only full bytes can be poped!");
        }
        Err("ERROR: Vector is empty!")
    }

    pub fn push_byte(&mut self, byte:u8) {
        if self.in_byte_pos == 8 {
            self.content.push(byte);
        } else {
            panic!("ERROR: Trying to push byte into BinVec where the last byte isn't completly filled");
        }
    }

    pub fn append_bin_vec(&mut self, other:BinVec) {
        if let None = self.appended_vector {
            self.appended_vector = Some(Box::new(other));
        } else {
            self.appended_vector.as_mut().unwrap().append_bin_vec(other);
        }
    }

    pub fn append_byte(&mut self, values:[bool;8]) {
        if self.in_byte_pos == 8 {                       //fresh byte can be used
            self.content.push(utils::bools_to_byte(values));
        } else {
            let mut byte = self.content.pop().unwrap();     //TODO: error handling
            let mut bit_mask:u8 = 0b1000_0000;
            bit_mask >> self.in_byte_pos +1;

            for i in 0..8-self.in_byte_pos {
                if values[i] {
                   byte = byte & bit_mask;
                }
                bit_mask >> 1;
            } 
            self.content.push(byte);

            byte = 0;
            bit_mask = 0b1000_0000; 
            for i in 8-self.in_byte_pos..8 {
                if values[i] {
                    byte = byte & bit_mask;
                }
                bit_mask >> 1;
            }

            self.content.push(byte);

        }
    }

    pub fn as_bytes<'a>(&'a self) -> BinVecBytewise<'a> {
        BinVecBytewise::new(self)
    }
}

struct BinVecBytewise<'a> {
    vector:Option<&'a BinVec>,
    cur_pos:usize,
}
impl<'a> BinVecBytewise<'a> {

    fn new(v:&'a BinVec) -> BinVecBytewise {
        BinVecBytewise{vector:Some(v), cur_pos:0}
    }
}

impl<'a> Iterator for BinVecBytewise<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.vector.is_some() {
            let cur_vec = &self.vector.unwrap();
            if self.cur_pos < cur_vec.content.len() {
                self.cur_pos += 1;
                return Some(cur_vec.content[self.cur_pos-1]);
            } else {
                self.vector = cur_vec.appended_vector.as_ref().map(|v| &**v);
                return self.next()
            }
        } else {
            None
        }
    }
}

