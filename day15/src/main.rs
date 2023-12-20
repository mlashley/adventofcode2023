use log::{debug, info, warn};
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::env;
use std::time::Instant;

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"{label}={focal}")]
struct Lens {
    label: String,
    focal: u64,
}

fn test() {
    debug_assert!(hash("HASH") == 52);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 1320
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 145
    );
}

fn hash(data: &str) -> u64 {
    let mut current = 0;
    for i in data.chars().map(|c| c as u64) {
        debug!("i={}", i);
        current += i;
        debug!("c+={}", current);
        current *= 17;
        debug!("c*={}", current);
        current %= 256;
        debug!("c%={}", current);
    }
    current
}

fn print_boxes(h: &HashMap<u64, Vec<Lens>>) -> u64 {
    let mut tot = 0;
    for box_number in 0..256 {
        let v = h.get(&box_number).unwrap();
        if !v.is_empty() {
            debug!("{}: {:?}", box_number, v);
            for (i, l) in v.iter().enumerate() {
                tot += (box_number + 1) * (1 + i as u64) * l.focal
            }
        }
    }
    tot
}

fn part1(data: &str) -> u64 {
    data.trim()
        .split(',')
        .filter(|y| !y.is_empty())
        .map(hash)
        .sum()
}
fn part2(data: &str) -> u64 {
    // I don't need a hashmap here - but the acronym calls for it ;-)
    // Holiday ASCII String Helper Manual Arrangement Procedure
    let mut hmap: HashMap<u64, Vec<Lens>> = HashMap::new();
    for box_number in 0..256 {
        hmap.insert(box_number, Vec::new());
    }

    data.trim()
        .split(',')
        .filter(|y| !y.is_empty())
        .for_each(|x| {
            if x.ends_with('-') {
                let label = x.get(0..x.len() - 1).unwrap();
                let box_number = hash(label);
                let v = hmap.get_mut(&box_number).unwrap();

                let _removed = v
                    .iter()
                    .position(|l| l.label == label)
                    .map(|e| v.remove(e))
                    .is_some();
            } else {
                match x.parse::<Lens>() {
                    Ok(s) => {
                        let box_number = hash(s.label.as_str());
                        let v = hmap.get_mut(&box_number).unwrap();
                        match v.iter().position(|l| l.label == s.label) {
                            None => { v.push(s)},
                            Some(i) => { v[i] = s;},
                        }
                    }
                    Err(e) => {
                        warn!("{:?} for {}", e, x);
                    }
                }
            }
        });
    print_boxes(&hmap)
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
    assert!(p1 == 514394);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 236358);
    info!("Completed in {} us", now.elapsed().as_micros());
}
