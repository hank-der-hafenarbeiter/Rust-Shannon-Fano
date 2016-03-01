use std::string;
use std::option;
use sfsym;
mod sfvec;


pub struct SFCodec {
    text:String,
    sym_table:sfvec::SFVec,
}

impl SFCodec {

    pub fn encode(&mut self, input_string:String) -> Option<String> {
        self.text = input_string;
        self.create_sym_table();
        self.create_code();
        //TODO reserve enough space for the resulting string
        Some(self.text.chars().fold(String::new(), |mut code, character| { let ref char_code = self.sym_table   .iter().find(|symbol| symbol.sym == character)
                                                                                                                .expect("Encountered unknown character!")
                                                                                                                .coding;
                                                                            code + &char_code}))
    }

    fn create_sym_table(&mut self) {

        for character in self.text.chars() {
            {
                let mut symbol = self.sym_table.iter_mut().find(|symbol| symbol.sym == character);
                if symbol.is_some() {
                    symbol.unwrap().count += 1;
                    continue;
                }
            }
            self.sym_table.push(sfsym::SFSym{sym:character, count:1, prob:0.0, coding:"".to_string()});
        }
 
    }

    fn create_code(&self) {
        
    }
    
    fn create_code_helper(&mut self, begin:usize, end:usize) {
        match sfvec::split(self.sym_table, begin, end) {
            Ok(x) =>{   for i in 0..x {
                            self.sym_table[i].coding.push_str("0");
                            self.create_code_helper(begin, x);
                        }
                        let length = self.sym_table.len();
                        for i in x+1..length-1 {
                            self.sym_table[i].coding.push_str("1");
                            self.create_code_helper(x+1, end);
                        }},
            Err(_) => {},
        }

    }
    
}
