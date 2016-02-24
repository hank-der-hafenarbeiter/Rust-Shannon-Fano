use std::ops::Add;

pub struct SFSym {
    pub	sym:char,
	pub count:i32,
    pub prob:f32,
	pub coding:String,
}


impl Add for SFSym {
    type Output = f32;

    fn add(self, op2:SFSym) -> f32 {
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
