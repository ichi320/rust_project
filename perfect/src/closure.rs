/// 7-10.closure
///
pub fn closure_sum() {
    let sum = |values: &Vec<i32>| {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    let values = vec![1, 2, 3, 4, 5];
    println!("sum = {}", sum(&values));
    println!("sum = {}", sum(&values));
}
