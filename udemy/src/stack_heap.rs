pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1,2,3,4];
    let v2 = vec![5,6,7,8];
    let mut v3 = vec![9,10];
    println!("Stach address of v1 is: {:p}", &v1);
    println!("Stach address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    v1.insert(1,10);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());


}
