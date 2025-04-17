use rand::Rng;
use std::collections::HashSet;

fn main() {
    let num_trials = 100000; // モンテカルロ法の試行回数
    let num_colors = 20; // ボールの種類
    let mut rng = rand::thread_rng();
    let mut total_draws = 0;

    for _ in 0..num_trials {
        let mut drawn_colors = HashSet::new();
        let mut num_draws = 0;
        while drawn_colors.len() < num_colors {
            let color = rng.gen_range(0..num_colors);
            drawn_colors.insert(color);
            num_draws += 1;
        }
        total_draws += num_draws;
    }

    let average_draws = total_draws as f64 / num_trials as f64;
    println!("全色そろえるために必要な平均引き数: {:.2}", average_draws);
}

