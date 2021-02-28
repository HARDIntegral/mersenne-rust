pub struct LargeInt{
    num_list: Vec<i32>,
    sign: bool,
}

pub fn build_int(list: Vec<i32>, sign: bool) -> LargeInt {
    LargeInt {
        num_list: list,
        sign,
    }
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

fn parser(num: &mut LargeInt) -> i32 {
    (*num).num_list.shrink_to_fit();
    let mut v = Vec::new();
    for i in 0..num.num_list.len() {
        v.push(num.num_list[i].to_string());
    }
    let s: String = v.into_iter().collect();
    s.parse::<i32>().unwrap()
}

fn num_digits(num: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let s: String = num.to_string();
    for x in s.chars() {
        v.push((x.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()) as i32);
    }
    v
}

pub fn mult(a: &mut LargeInt, b: &mut LargeInt) -> Vec<i32> {
    // To check if numbers are small enough
    if (*a).num_list.len() < 5 && (*b).num_list.len() < 5 {
        let a_num: i32= parser(a);
        let b_num: i32 = parser(b);
        let res = a_num * b_num;
        // Make this in the future
        num_digits(res)
    } else {
        vec![3, 3]
    }
}
