use rand::Rng;
use std::collections::HashSet;

fn monte_carlo_simulation(trials: usize, balls: usize, draws: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let mut success_count = 0;

    for _ in 0..trials {
        let mut drawn_balls = HashSet::new();

        for _ in 0..draws {
            let ball = rng.gen_range(0..balls);
            drawn_balls.insert(ball);
        }

        if drawn_balls.len() == balls {
            success_count += 1;
        }
    }

    success_count as f64 / trials as f64
}

fn main() {
    let trials = 100000;  // シミュレーションの回数
    let balls = 10;       // ボールの種類数
    let draws = 15;       // 引く回数

    let probability = monte_carlo_simulation(trials, balls, draws);
    println!("全色そろう確率: {:.6}", probability);
}

