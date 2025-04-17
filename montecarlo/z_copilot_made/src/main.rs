// Copilot made
// 
fn main() {
    mytest2();
}

use rand::Rng;
use std::collections::HashSet;

fn mytest2() {
    let total_simulations = 1_000_000; // シミュレーション回数
    let total_colors = 10; // ボールの色の数
    let total_draws = 15; // 取り出す回数
    let mut rng = rand::rng();
    let mut successful_trials = 0;

    // モンテカルロ法でシミュレーションを繰り返す
    for _ in 0..total_simulations {
        let mut drawn_balls = HashSet::new(); // 取り出した色を保持するためのセット
        for _ in 0..total_draws {
            let ball = rng.random_range(0..total_colors); // 0から9の範囲でランダムなボールを取り出す
            drawn_balls.insert(ball); // 色をセットに追加
            if drawn_balls.len() == total_colors {
                successful_trials += 1; // すべての色が揃ったら成功
                break; // 他のボールを取り出す必要はないので終了
            }
        }
    }

    // 全色が揃う確率を計算
    let probability = successful_trials as f64 / total_simulations as f64;
    println!("{}回の試行で全色揃う確率: {:.6}", total_draws, probability);
}

/*
fn mytest1() {
    const KIND: usize = 10;
    let mut s: u64 = 0;
    for _ in 0..10000 {

        let mut hash_table: [usize; KIND] = [0; KIND];

        for i in 0..10000 {
            let mut rng = rand::rng();
            let a: usize = rng.random_range(0..KIND);
            hash_table[a] += 1;
            if hash_table.iter().all(|&x| x != 0) {
                s += i;
                break;
            }
        }
        //println!("hash_table: {:?}", hash_table);
    }
    println!("average try count: {}", s / 10000);

}
*/


