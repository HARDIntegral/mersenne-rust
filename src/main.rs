mod large;

fn main() {
    let mut a = large::build_int(vec![1, 2, 3, 4], true);
    let mut b = large::build_int(vec![5, 6, 7, 8], true);
    let ans = large::mult(&mut a, &mut b);
    println!("{:?}", ans);
    for i in 0 ..10 {
        if is_prime(i) {
            //println!("{}", i);
        }
    }
}

fn is_prime(x: i64) -> bool {
    let mut b = true;

    if x < 2  {
        b = false;
    } else {
        let limit = ((x as f64).sqrt()) as i64 + 1;
        for i in 2..limit {
            if x % i == 0{
                b = false;
                break;
            }
        }
    }
    
    b
}