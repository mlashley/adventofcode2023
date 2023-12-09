use log::{debug, error, info, log_enabled, warn, Level};
use parse_display::{Display, FromStr};
use std::env;
use std::time::Instant;

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"Time: {times}")]
struct TImes {
    times: String,
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"Distance: {distances}")]
struct Distances {
    distances: String,
}

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 288
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

fn part1(data: &str) -> usize {
    let lines: Vec<_> = data.split('\n').filter(|y| !y.is_empty()).collect();
    let timething = lines[0].parse::<TImes>().unwrap();
    let distancething = lines[1].parse::<Distances>().unwrap();
    let times: Vec<_> = timething
        .times
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<_> = distancething
        .distances
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    debug!("T: {:?} D: {:?}", times, distances);

    let mut winning_multipler = 1;
    for (i, t) in times.into_iter().enumerate() {
        let mut winning_this = 0;
        for hold in 1..t {
            let my_distance = (t - hold) * hold;
            debug!("H: {} D: {}", hold, my_distance);
            if my_distance > distances[i] {
                winning_this += 1;
            }
        }
        winning_multipler *= winning_this;
    }

    winning_multipler
}
fn part2(data: &str) -> usize {
    888
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part1: {}", p1);
    assert!(p1 == 131376);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
