use rand::Rng;
use std::collections::HashSet;

fn main() {
    let num_trials = 100000; // モンテカルロ法の試行回数
    let num_colors = 10; // ボールの種類
    let num_draws = 15; // 抽出回数
    let mut rng = rand::thread_rng();
    let mut successful_trials = 0;

    for _ in 0..num_trials {
        let mut drawn_colors = HashSet::new();
        for _ in 0..num_draws {
            let color = rng.gen_range(0..num_colors);
            drawn_colors.insert(color);
        }
        if drawn_colors.len() == num_colors {
            successful_trials += 1;
        }
    }

    let probability = successful_trials as f64 / num_trials as f64;
    println!("全色そろう確率: {:.5}", probability);
}

