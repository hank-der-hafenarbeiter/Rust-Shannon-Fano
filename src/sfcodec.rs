use std::cmp::Ordering;
use std::thread;
use std::sync::mpsc::channel;
use sfsym;
use sfvec;
use sfvec::Mergeable;

#[derive(Debug)]
pub struct SFCodec {
    text:String,
    sym_table:sfvec::SFVec,
    num_threads:usize,
}
        
impl SFCodec {

    pub fn new() -> SFCodec {
        SFCodec{ text:"".to_string(), sym_table:Vec::new(), num_threads:0}
    }
    
    pub fn multithread_with(&mut self, i:usize) {
        self.num_threads = i;
    }

    pub fn encode(&mut self, input_string:String) -> Option<String> {
        self.text = input_string;
        self.create_sym_table();
        self.create_code();
        //TODO reserve enough space for the resulting string
        Some(self.text.chars().fold(String::new(), |code, character| { let ref char_code = self.sym_table   .iter().find(|symbol| symbol.sym == character)
                                                                                                                .expect("Encountered unknown character!")
                                                                                                                .coding;
                                                                            code + &char_code}))
    }

    fn parse_text(&mut self) {
        let vec_handles = Vec::with_capacity(self.num_threads);
        
        let remainder = self.text.len() % self.num_threads;
        let part_size = (self.text.len() - remainder)/self.num_threads;
        let mut begin = 0;
        let mut end = part_size;
        for i in 0..self.num_threads {
            vec_handles.push(thread::spawn(move || {
                let t_sym_table = sfvec::SFVec::new();
                for character in self.text[begin..end].chars() {
                    {
                        let symbol = t_sym_table.iter_mut().find(|symbol| symbol.sym == character);
                        if symbol.is_some() {
                            symbol.unwrap().count += 1;
                            continue;
                        }
                    }
                    t_sym_table.push(sfsym::SFSym{sym:character, count:1, prob:0.0, coding:"".to_string()});
                }
                t_sym_table 
            }));
            begin = end;
            end += part_size;
        }

        let mut t_sym_table = Self::parse_text_helper(&self.text[end..]).unwrap();
        
        for thread in vec_handles.iter() {
            t_sym_table.merge(thread.join().unwrap());
        }

        self.sym_table = t_sym_table;
        
    }
    
    fn parse_text_helper(part_text:&str) -> Result<sfvec::SFVec, String> {
        let t_sym_table = sfvec::SFVec::new();
        for character in part_text.chars() {
            {
                let symbol = t_sym_table.iter_mut().find(|symbol| symbol.sym == character);
                if symbol.is_some() {
                    symbol.unwrap().count += 1;
                    continue;
                }
            }
            t_sym_table.push(sfsym::SFSym{sym:character, count:1, prob:0.0, coding:"".to_string()});
        }
        Ok(t_sym_table)
    } 
    fn create_sym_table(&mut self) {
        //for character in self.text.chars() {
        //    {
        //        let symbol = self.sym_table.iter_mut().find(|symbol| symbol.sym == character);
        //        if symbol.is_some() {
        //            symbol.unwrap().count += 1;
        //            continue;
        //        }
        //    }
        //    self.sym_table.push(sfsym::SFSym{sym:character, count:1, prob:0.0, coding:"".to_string()});
        //}

        self.parse_text();

        let text_len = self.text.chars().fold(0,|acc, _| acc + 1);
        for sym in self.sym_table.iter_mut() {
            sym.prob = (sym.count as f64)/(text_len as f64);
        }

        self.sym_table.sort_by(|a,b|{   let diff = b.prob -a.prob;  //descending order needed
                                        if diff == 0.0 {
                                            Ordering::Equal
                                        } else if diff < 0.0 {
                                            Ordering::Less
                                        } else {
                                            Ordering::Greater
                                        }});


    }

    fn create_code(&mut self) {
        let end = self.sym_table.len();
        self.create_code_helper(0, end);
    }
    
    fn create_code_helper(&mut self, begin:usize, end:usize) {
        match sfvec::split(&self.sym_table, begin, end) {
            Ok(x) =>{   
                        for i in begin..x {
                            self.sym_table[i].coding.push_str("0");
                            
                        }
                        if begin+1 < x {
                            self.create_code_helper(begin, x); 
                        }

                        for i in x..end {
                            self.sym_table[i].coding.push_str("1");
                        }
                        if x+1 < end {
                            self.create_code_helper(x, end);
                        }
                    },
            Err(x) => {println!("Error: {:?}", x)},
        }

    }
    
}
