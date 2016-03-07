use utils;

struct BinVec {
    content:Vec<u8>,
    in_byte_pos:usize, //gives the first free position in the last byte of the vector
}

impl BinVec {

    pub fn new () -> BinVec {
        BinVec{ content:Vec::new(), in_byte_pos:0}
    }

    pub fn push(&mut self, value:bool) {
        
    }

    pub fn append_byte(&mut self, values:[bool;8]) {
        if in_byte_pos == 0 {                       //fresh byte can be used
            content.push(bools_to_byte(values));
        } else {
            let mut t_slice = values[0..8-in_byte_pos];
            let mut content = self.content.pop();
            let mut bit_mask:u8 = 0b00000001;
            bit_mask << in_byte_pos +1;

            for x in t_slice {
                if x {
                   content = (content & bit_mask);
                }
                bit_mask << 1;
            } 
        }
    }
}
