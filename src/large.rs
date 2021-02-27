struct largeInt {
    num_lists: Vec<i32>,
    sign: bool,
}

// Karatsuba Algorithm
fn take_first_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(0..half - 1).collect()
}

fn take_second_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(half..).collect()
}

pub fn exp2(a: *mut largeInt, b: *mut largeInt) {
    // To check if numbers are small enough
    if a.len() < 120 && b.len() < 120 {
        let mut a_num = a.parse(i128).except("Parse Unsuccessful");
        let mut b_num = b.parse(i128).except("Parse Unsuccessful");
        let res = a * b;
        // TODO: figure out how to convert an int to a vector
    }
}
