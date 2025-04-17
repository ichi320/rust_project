fn main() {
    println!("Hello, world!");
    let n: usize = 3;
    let m: i32 = 9;
    let k:[i32; 3] = [1, 3, 5];
    
    let mut result = false;

    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                if binary_search(m - k[a] - k[b] - k[c], n, &k) {
                    result = true;
                // for d in 0..n {
                //     if (k[a] + k[b] + k[c] + k[d] ) == m {
                //         result = true;
                //     }
                }
            }
        }
    }
    println!("{result}");
}

fn binary_search(x: i32, n: usize, k: &[i32]) -> bool {
    let mut l = 0;
    let mut r = n;
    while r - l > 0 {
        let i = (l + r) / 2;
        if k[i] == x {
            return true;
        } else if k[i] < x {
            l = i + 1;
        } else {
            r = i;
        }
    }
    false
}
