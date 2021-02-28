// Addition
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn add(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    vec![3, 3] // Placeholder
}

// Multiplication
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn take_first_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(0..half - 1).collect()
}

fn take_second_half<T>(v: &mut Vec<T>) -> Vec<T> {
    let half = v.len() / 2;
    v.drain(half..).collect()
}

// This funtion parses a vector containing a number in the form of a vector to a 32-bit integer
fn parser(num: &mut Vec<i32>) -> i32 {
    (*num).shrink_to_fit();
    let mut v = Vec::new();
    for i in 0..num.len() {
        v.push(num[i].to_string());
    }
    let s: String = v.into_iter().collect();
    s.parse::<i32>().unwrap()
}

// This funtion converts a 32-bit integer to a vector containing that number in the form of its digits 
fn num_digits(num: i32) -> Vec<i32> {
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

// Karatsuba Algorithm
pub fn mult(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    // To check if numbers are small enough
    if (*a).len() < 5 && (*b).len() < 5 {
        let a_num: i32= parser(a);
        let b_num: i32 = parser(b);
        let res = a_num * b_num;
        // Make this in the future
        num_digits(res)
    } else {
        let mut a_first: Vec<i32> = take_first_half(a);
        let mut a_second: Vec<i32> = take_second_half(a);
        
        let mut b_first: Vec<i32> = take_first_half(b);
        let mut b_second: Vec<i32> = take_second_half(b);

        let mult_front: Vec<i32> = mult(& mut a_first, & mut b_first);
        let mult_back: Vec<i32> = mult(& mut a_second, & mut b_second);        

        vec![3, 3] // Placeholder
    }
}
