use std::cmp::Ordering;

extern crate time;
extern crate crossbeam;

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
        SFCodec{ text:"".to_string(), sym_table:Vec::new(), num_threads:1}
    }

    pub fn multithread_with(&mut self, i:usize) {
        if i > 0 {
            self.num_threads = i;
        } else {
            self.num_threads = 1;
        }
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
        let mut vec_handles = Vec::with_capacity(self.num_threads);
        let mut begin = 0;
        let mut end = 0;
        let mut t_sym_table = Vec::new(); 

        if self.num_threads == 0 {
            self.num_threads = 1
        }

        let start = time::PreciseTime::now();

        let remainder = self.text.len() % self.num_threads;
        let part_size = (self.text.len() - remainder)/self.num_threads;
        end = part_size;
        crossbeam::scope(|scope| {
            for i in 0..self.num_threads-1 {
                let part_text = &self.text[begin..end];
                    
                vec_handles.push(scope.spawn(move || SFCodec::parse_text_helper(&part_text)));

                begin = end;
                end += part_size;
            }

            t_sym_table = SFCodec::parse_text_helper(&self.text[begin..self.text.len()]);
            for thread in vec_handles.into_iter() {
                let other_table = thread.join();
                t_sym_table.merge(other_table);
            }
                
            });
         

        self.sym_table = t_sym_table;
        let duration = start.to(time::PreciseTime::now());
        println!("{} threads took {}", self.num_threads, duration);
    }

    fn parse_text_helper(part_text:&str) -> sfvec::SFVec { 
        let mut t_sym_table = sfvec::SFVec::new();
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
        t_sym_table 

    } 
    fn create_sym_table(&mut self) {

        self.parse_text();

        let text_len = self.sym_table.iter().fold(0, |acc,x| acc + x.count);
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
