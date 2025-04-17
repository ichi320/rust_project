/// ## 7-8.Option<T>
///
fn div(v1: i32, v2: i32) -> Option<i32> {
    if v2 == 0 {
        return None;
    }
    let r = (v1 / v2) as i32;
    Some(r)
}

pub fn use_div() {
    let x = 10;
    let y = 0;
    let r = match div(x, y) {
        None => "couldn't divide.".to_owned(),
        Some(result) => format!("{} / {} = {}", x, y, result)
    };
    println!("{}", r);
}
