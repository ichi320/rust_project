use std::time::Instant;
use rand::Rng;
use std::collections::HashMap;

fn monte_carlo_simulation(trials: usize, balls: usize) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let mut total_draws = 0;
    let mut max_counts_total = 0;

    for _ in 0..trials {
        let mut ball_counts = vec![0; balls]; // 各ボールの引かれた回数をカウントするベクター
        let mut drawn_balls = HashMap::new();
        let mut draws = 0;

        while drawn_balls.len() < balls {
            let ball = rng.gen_range(0..balls);
            ball_counts[ball] += 1;
            drawn_balls.insert(ball, true); // 引いたボールをセットに追加
            draws += 1;
        }

        // 最も多く引かれたボールの数を求める
        let max_count = *ball_counts.iter().max().unwrap();
        total_draws += draws;
        max_counts_total += max_count;
    }

    // 平均引き回数と、最も多く引かれたボールの平均回数
    let avg_draws = total_draws as f64 / trials as f64;
    let avg_max_count = max_counts_total as f64 / trials as f64;

    (avg_draws, avg_max_count)
}

fn main() {
    // 計測開始
    let start = Instant::now();
    
    let trials = 100000;  // シミュレーションの回数
    let balls = 20;       // ボールの種類数

    let (avg_draws, avg_max_count) = monte_carlo_simulation(trials, balls);
    println!("全種類のボールを揃えるために必要な平均引き回数: {:.2}", avg_draws);
    println!("全種類のボールが揃ったときの一番多いボールの平均回数: {:.2}", avg_max_count);

    let duration = start.elapsed();
    println!("実行時間: {:?}", duration);
}

