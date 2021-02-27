fn main() {
    for i in 0..100 {
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