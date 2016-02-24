use sfsym::SFSym;


pub type SFVec = Vec<SFSym>;

pub fn split(input_vec:SFVec) ->  usize {
    let total_prob:f32 = input_vec.iter().fold(0.0,|sum:f32,x| sum+x.prob);
    let mut prob_count:f32 = 0.0;
    let mut result:usize = 0;
    for (pos, ele) in input_vec.iter().enumerate(){
        prob_count += ele.prob;
        if prob_count > total_prob/2.0 {
            result = pos;
            break;
        }
    }
    result
}


#[test]
fn split_in_right_place() {
    let test_vec:SFVec = vec![   SFSym{sym:'a', count:6, prob:3.0/10.0, coding:"0".to_string()},
            SFSym{sym:'b', count:2, prob:3.0/10.0, coding:"1".to_string()},
            SFSym{sym:'c', count:1, prob:2.0/10.0, coding:"01".to_string()},
            SFSym{sym:'d', count:1, prob:2.0/10.0, coding:"10".to_string()},];
    assert!(split(test_vec) == 1);
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
