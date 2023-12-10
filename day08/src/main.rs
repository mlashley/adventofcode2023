// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use log::{debug, error, info, log_enabled, warn, Level};
use parse_display::{Display, FromStr};
use std::env;
use std::time::Instant;
use std::collections::HashMap;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 2
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample2.txt")
                .unwrap()
                .as_str()
        ) == 6
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"{from} = ({left}, {right})")]
struct Node {
    from: String,
    left: String,
    right: String,
}

fn part1(data: &str) -> usize {
    let chunks: Vec<_> = data.split("\n\n").filter(|y| !y.is_empty()).collect();
    let directions = chunks[0].chars().cycle(); // repeating iterator
    
    // Build the network
    let mut node_hashmap: HashMap<String, Node> = HashMap::new();
    chunks[1]
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Node>().unwrap())
        .map(|x| { node_hashmap.insert(x.from.clone(),x.clone()) })
        .for_each(drop);
    debug!("MAP: {:?}", node_hashmap);

    // Walk it
    let mut current = "AAA";
    let mut count = 0;
    for d in directions {
        current = match d {
            'L' => &node_hashmap.get(current).unwrap().left,
            'R' => &node_hashmap.get(current).unwrap().right,
            _   => { error!("Fuuu"); "Fuu" },
        };      
        count +=1;
        if current == "ZZZ" { break; }
    }
    count
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
    assert!(p1 == 22199);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
