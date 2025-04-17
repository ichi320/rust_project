use std::io::{self, Write};  // 入力と出力のためのモジュール
use std::str::FromStr;  // 文字列から型への変換

// 入力関数：1行読み込み、スペース区切りで整数として解析
fn read_line() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim()
         .split_whitespace()
         .map(|x| x.parse().unwrap())
         .collect()
}

fn main() {
    // ここで入力を受け取ります
    let n: usize = read_line()[0] as usize; // 例：1行目の整数n
    let a: Vec<i64> = read_line();         // 例：2行目の整数列

    // ここでアルゴリズムを記述
    // 例: 配列の合計を計算
    let sum: i64 = a.iter().sum();

    // 結果を標準出力に出力
    println!("{}", sum);
}

