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

trait Mergeable {
    fn merge(&mut self, Self);
}
impl Mergeable for SFVec {

    pub fn merge(&mut self, r_op:SFVec) {
        for r_sym in r_op.iter() {
            if let Some(search_res) = self.mut_iter().find(|l_sym| l_sym.sym == r_sym.sym) {
                search_res.unwrap().count += 1;
                continue;
            }
            self.push(r_sym);
        }
    }    
}

#[test]
fn split_in_right_place() {
    let test_vec:SFVec = vec![   SFSym{sym:'a', count:6, prob:3.0/10.0, coding:"0".to_string()},
            SFSym{sym:'b', count:2, prob:3.0/10.0, coding:"1".to_string()},
            SFSym{sym:'c', count:1, prob:2.0/10.0, coding:"01".to_string()},
            SFSym{sym:'d', count:1, prob:2.0/10.0, coding:"10".to_string()},];
    let length = test_vec.len();
    assert!(split(test_vec, 0, length-1) == Ok(1));
}

#[test]
fn test_for_return() {
    let mut result:usize = 0;

    for (pos, _) in (0..100).enumerate() {
        result = pos;
        println!["{}",result];
    }

    assert!(result != 0);
}
