fn main() {
    let text = "Hello, world!".to_string();
    
    let count:usize = first_word(&text);
    println!("text letter count is: {}", count);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

