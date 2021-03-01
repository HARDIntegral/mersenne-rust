use std::{cmp, convert::TryInto};

mod helper;

// Addition
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn add(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    // To figure out which number is bigger and extend the shorter of the two
    if cmp::max(a.len(), b.len()) == a.len() {
        let target_a: i32 = a.len().try_into().unwrap();
        let target_b: i32 = a.len().try_into().unwrap();

        helper::extend(a, target_a);
        helper::extend(b, target_b);
    } else {
        let target_a: i32 = (*b).len().try_into().unwrap();
        let target_b: i32 = (*b).len().try_into().unwrap();

        helper::extend(a, target_a);
        helper::extend(b, target_b);
    }

    let limit: i32 = a.len().try_into().unwrap(); 
    a.reverse();
    b.reverse();

    let mut carry: Vec<i32> = vec![0];
    let mut sum: Vec<i32> = Vec::new();

    // Adds individual digits much like how computers add numbers
    for i  in 0..limit {
        let dig_ans = a[i as usize] + b[i as usize] + carry[i as usize];
        // To check for overflow
        if dig_ans > 9 {
            carry.push((dig_ans)/10 as i32);
            sum.push(dig_ans - 10);
        } else {
            sum.push(dig_ans);
            carry.push(0);
        }
    }
    // To remove the extra 0 at the front of the number
    sum.pop();
    sum.reverse();

    sum
}

// Subtraction
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sub(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {


    vec![3, 3] //Placeholder
}

// Multiplication
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Karatsuba Algorithm
pub fn mult(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    // To check if numbers are small enough
    if a.len() < 5 && b.len() < 5 {
        let a_num: i32= helper::parser(a);
        let b_num: i32 = helper::parser(b);
        let res = a_num * b_num;
        // Make this in the future
        helper::num_digits(res)
    } else {
        let mut a_first: Vec<i32> = helper::take_first_half(a);
        let mut a_second: Vec<i32> = helper::take_second_half(a);
        
        let mut b_first: Vec<i32> = helper::take_first_half(b);
        let mut b_second: Vec<i32> = helper::take_second_half(b);

        let mut mult_front: Vec<i32> = mult(& mut a_first, & mut b_first);
        let mut mult_back: Vec<i32> = mult(& mut a_second, & mut b_second);   
        
        let mut add_first: Vec<i32> = add(&mut a_first, &mut a_second);
        let mut add_second: Vec<i32> = add(&mut b_first, &mut b_second);

        let mult_mid: Vec<i32> = mult(&mut add_first, &mut add_second);

        vec![3, 3] // Placeholder
    }
}
