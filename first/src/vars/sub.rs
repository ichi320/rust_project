pub fn test() {
    println!("This is a test function.");

    const C:u32 = 111;
    
    println!("Address of C: {:p}", &C);
    let i1 = 1;
    let i2 = 2;
    println!("Address of i1: {:p}", &i1);
    println!("Address of i2: {:p}", &i2);
}
