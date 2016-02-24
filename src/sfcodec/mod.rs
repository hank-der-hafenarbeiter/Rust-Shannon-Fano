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

        //self.text   .chars()
        //            .map(|character| self.sym_table .iter()
        //                                            .find(|symbol| symbol.sym == character)
        //                                            .unwrap_or_else(|| {self.sym_table.push(sfsym::SFSym{sym:character, count:0, prob:0.0, coding:"".to_string()});
        //                                                                self.sym_table.iter().last().expect("Unexpectedly encountered empty sym_table!")})
        //                                            .count += 1)
        //            .collect();
        
        for character in self.text.chars() {
            self.sym_table.iter()
                            .find(|symbol| symbol.sym == character)
                            .unwrap_or_else(|| {self.sym_table.push(sfsym::SFSym{sym:character, count:0, prob:0.0, coding:"".to_string()});
                                                self.sym_table.iter().last().expect("Unexpectedly encountered empty sym_table!")})
                            .count += 1;
        }
 
    }

    fn create_code(&self) {

    }

    
}
