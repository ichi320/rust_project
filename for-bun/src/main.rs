use quanta::Clock;
use std::time::Duration;

fn main() {
    let clock = Clock::new();
    let start = clock.now();
    let time_limit = Duration::new(3, 0);
    let num = 0;
    for _ in 0..1000000000 {
        let num = num + 1;
        let stop = clock.now();
        let time = stop.duration_since(start);
        if time > time_limit {
            println!("Time over!!");
            break;
        }
    }
    let stop = clock.now();
    let time = stop.duration_since(start);
    println!("Time is: {:?}", time);
}
