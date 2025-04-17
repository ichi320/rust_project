fn main() {
    let mut a = 1;
    let mut i = 1;
    
    loop {
        i += 1;
        a += i;
        println!("a is: {}", a);
        if a > 5 {
            break;
        }
    }
}

