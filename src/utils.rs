/// Mask of the value bits of a continuation byte
const CONT_MASK: u8 = 0b0011_1111;
/// Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte
const TAG_CONT_U8: u8 = 0b1000_0000;


 fn is_acceptable_index(text:&String, index: usize) -> bool {
    if index == text.len() {
        true
    } else {
        text.as_bytes().get(index).map_or(false, |byte| {
            // check it's not a
            // continuation byte
            (byte & !CONT_MASK) != TAG_CONT_U8
        })
    }
}

pub fn find_previous_char_boundary(text:&String, index:usize) -> usize {
    let mut index = index;
    while !is_acceptable_index(text, index) {
        index -=1;
    }
    index
}

