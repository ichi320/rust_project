mod sub;

pub fn run() {
    println!("This is a run.");
    sub::test();
    let guess: i32 = "42".parse().expect("Not a number.");
    println!("number is: {}", guess);
}

