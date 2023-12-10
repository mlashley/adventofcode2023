use std::time::Instant;
use std::env;
use log::{debug, info};
use itertools::Itertools;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 114
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 2
    );
}

fn extrapolate(v: Vec<i64>, is_backward: bool) -> i64 {
    let mut r = Vec::new();
    for (a, b) in v.iter().tuple_windows() {
        r.push(b - a);
    }
    if r.iter().all(|&b| b == 0) {
        debug!("Recurse: {:?}", v[0]);
        v[0]
    } else {
        let e = extrapolate(r.clone(), is_backward);
        debug!("{:?} => {}", r, e);
        if is_backward {
            return v.first().unwrap() - e;
        } else {
            return v.last().unwrap() + e;
        }
    }
}

fn partn(data: &str, is_part2: bool) -> i64 {
    data.split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| {
            extrapolate(
                x.split(' ').map(|x| x.parse::<i64>().unwrap()).collect(),
                is_part2,
            )
        })
        .sum()
}
fn part1(data: &str) -> i64 {
    partn(data, false)
}
fn part2(data: &str) -> i64 {
    partn(data, true)
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
    assert!(p1 == 1904165718);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 964);
    info!("Completed in {} us", now.elapsed().as_micros());
}
