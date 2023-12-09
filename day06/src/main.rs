use log::{debug, info};
use parse_display::{Display, FromStr};
use std::env;
use std::time::Instant;

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"Time: {times}")]
struct Times {
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
        ) == 71503
    );
}

fn part1(data: &str) -> usize {
    let lines: Vec<_> = data.split('\n').filter(|y| !y.is_empty()).collect();
    let timething = lines[0].parse::<Times>().unwrap();
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
                debug!("WINNER WINNER CHICKEN DINNER");
            }
        }
        winning_multipler *= winning_this;
    }

    winning_multipler
}
fn part2(data: &str) -> u64 {
    let lines: Vec<_> = data.split('\n').filter(|y| !y.is_empty()).collect();
    let timething = lines[0].parse::<Times>().unwrap();
    let distancething = lines[1].parse::<Distances>().unwrap();
    let time = timething.times.replace(' ', "").parse::<u64>().unwrap();

    let distance = distancething
        .distances
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    debug!("T: {:?} D: {:?}", time, distance);

    // This should probably pick a mid-point and iterate (by halves a la git-bisect, or Newton-Raphson) to find the zero-crossing in O(logN), rather than O(N),
    // but I'm not doing a coding interview and still catching up from vacation, and anyway optimizing down 15ms runtime here is pointless optimization.
    let mut winning_start = 0;
    for hold in 1..time {
        let my_distance = (time - hold) * hold;
        debug!("H: {} D: {}", hold, my_distance);
        if my_distance > distance {
            winning_start = hold;
            info!("start: {}", winning_start);
            break;
        }
    }

    let mut winning_end = 0;
    for hold in (1..time).rev() {
        let my_distance = (time - hold) * hold;
        debug!("H: {} D: {}", hold, my_distance);
        if my_distance > distance {
            winning_end = hold;
            info!("end: {}", winning_end);
            break;
        }
    }
    1 + winning_end - winning_start
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
    assert!(p2 == 34123437);
    info!("Completed in {} us", now.elapsed().as_micros());
}
