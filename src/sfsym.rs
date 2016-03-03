use std::cmp;
use std::ops::Add;
use std::option::Option;

#[derive(Debug)]
pub struct SFSym {
    pub	sym:char,
	pub count:i32,
    pub prob:f64,
	pub coding:String,
}

impl cmp::Ord for SFSym {
    fn cmp(&self, other:&Self) -> cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl cmp::PartialOrd for SFSym {

    fn partial_cmp(&self, other:&Self) -> Option<cmp::Ordering> {
        let cmp = self.count - other.count;
        if cmp < 0 {
            return Some(cmp::Ordering::Less)
        }
        if cmp == 0 {
            return Some(cmp::Ordering::Equal)
        }
        Some(cmp::Ordering::Greater)
    }

    fn lt(&self, other:&Self) -> bool {
        self.count < other.count
    }

    fn le(&self, other:&Self) -> bool {
        self.count <= other.count
    }

    fn gt(&self, other:&Self) -> bool {
        self.count > other.count
    }

    fn ge(&self, other:&Self) -> bool {
        self.count >= other.count
    }
}

impl cmp::PartialEq for SFSym {
    fn eq(&self, other:&Self) -> bool {
        self.count == other.count
    }

    fn ne(&self, other:&Self) -> bool {
        self.count != other.count
    }
}

impl cmp::Eq for SFSym {}

impl Add for SFSym {

    type Output = f64;

    fn add(self, op2:SFSym) -> f64 {
        self.prob+op2.prob
    }
}

impl Default for SFSym {
    fn default() -> SFSym {SFSym {  sym:'_',
                                    count:0,
                                    prob:0.0,
                                    coding:"".to_string()}
    }
}
