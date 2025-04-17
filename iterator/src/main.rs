fn main() {
    println!("Hello, world!");
    println!("triangle({})", triangle(10));
}

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

