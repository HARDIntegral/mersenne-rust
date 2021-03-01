use std::convert::TryInto;

// This funtion parses a vector containing a number in the form of a vector to a 32-bit integer
pub fn parser(num: &mut Vec<i32>) -> i32 {
    (*num).shrink_to_fit();
    let mut v = Vec::new();
    for i in 0..num.len() {
        v.push(num[i].to_string());
    }
    let s: String = v.into_iter().collect();
    s.parse::<i32>().unwrap()
}

// This funtion converts a 32-bit integer to a vector containing that number in the form of its digits 
pub fn num_digits(num: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let s: String = num.to_string();
    for x in s.chars() {
        v.push(
            (x
                .to_string()
                .chars().map(|c| c.to_digit(10)
                .unwrap())
                .sum::<u32>()
            ) as i32
        );
    }
    v
}

// This funtion is used to make sure both input numbers are of equal length
pub fn extend(num: &mut Vec<i32>, targert: i32) -> Vec<i32> {
    (*num).reverse();
    let mut i: i32 = (*num).len().try_into().unwrap();
    while i < (targert + 1) {
        (*num).push(0);
        i += 1;
    }
    (*num).reverse();
    num.to_vec()
}

pub fn take_first_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(0..half - 1).collect()
}

pub fn take_second_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(half..).collect()
}