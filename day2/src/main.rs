// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::time::Instant;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 999
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
    999
}
fn part2(data: &str) -> usize {
    888
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str()
    );
    println!("Part1: {}", p1);
    assert!(p1 == 999);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 888);
    println!("Completed in {} us", now.elapsed().as_micros());
}
