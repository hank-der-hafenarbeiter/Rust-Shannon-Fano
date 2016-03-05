use sfsym::SFSym;

pub type SFVec = Vec<SFSym>;

pub fn split(input_vec:&SFVec, begin:usize, end:usize) ->  Result<usize,String> {
    
    if begin < end && end <= input_vec.len() {
        let mut result:usize = begin+1;
        let mut total_prob = 0.0;

        for x in begin..end {
            total_prob += input_vec[x].prob;
        }
        
        let mut prob_count:f64 = input_vec[begin].prob;
        let mut diff1:f64 = (total_prob/2.0) - prob_count;
        let mut diff2:f64 = 1.0;

        while diff1.abs() < diff2.abs() {
            prob_count += input_vec[result].prob;
            diff2 = diff1;
            diff1 = (total_prob/2.0) - prob_count; 
            result += 1;
        }

        result -= 1; 

        Ok(result)
    }
    else {
        Err("Invalid Parameters!".to_string())
    }
}

pub trait Mergeable {
    fn merge(&mut self, Self);
}

impl Mergeable for SFVec {

    fn merge(&mut self, r_op:SFVec) {
        for r_sym in r_op.iter() {
            if let Some(search_res) = self.iter_mut().find(|ref l_sym| l_sym.sym == r_sym.sym) {
                search_res.count += r_sym.count;
                continue;
            } 
            self.push(r_sym.clone());
        }
    }    
}


