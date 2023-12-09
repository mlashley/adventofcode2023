// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Thing {
    value: Option<u32>,
    symbol: char,
    is_partnumber: bool,
}
impl Thing {
    fn new(symbol: char) -> Self {
        let v = symbol.to_digit(10);

        Self {
            value: v,
            symbol,
            is_partnumber: false,
        }
    }
}

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 4361
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 467835
    );
}
fn print_map(map: &HashMap<(i64, i64), Thing>) {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let xmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let ymin = *map.keys().map(|(_, y)| y).min().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&(x, y)) {
                print!("{}", map.get(&(x, y)).unwrap().symbol);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_map_val(map: &HashMap<(i64, i64), Thing>) {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let xmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let ymin = *map.keys().map(|(_, y)| y).min().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&(x, y)) {
                print!("{}", map.get(&(x, y)).unwrap().value.unwrap_or_else(|| 0));
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn is_partnumber(map: &HashMap<(i64, i64), Thing>, x: i64, y: i64) -> bool {
    let num_len = map
        .get(&(x, y))
        .unwrap()
        .value
        .unwrap()
        .checked_ilog10()
        .unwrap_or(0)
        + 1;
    // println!("Len {} at {},{}",num_len,x,y);
    let xmin = x - (i64::from(num_len));

    for xx in xmin..x + 2 {
        for yy in y - 1..y + 2 {
            let e = map.get(&(xx, yy));
            match e
                .unwrap_or_else(|| &Thing {
                    value: Some(99),
                    symbol: '.',
                    is_partnumber: false,
                })
                .value
            {
                None => return true,
                Some(x) => {}
            }
        }
    }
    false
}

fn merge_map(map: &mut HashMap<(i64, i64), Thing>) {

    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let xmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let ymin = *map.keys().map(|(_, y)| y).min().unwrap();
    // Merge the numbers.
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&(x, y)) {
                let thing_value = map.get(&(x, y)).unwrap().value;
                // println!("{},{} is value ",x,y);

                if thing_value != None {
                    // Is this a run-on-value?
                    if map.contains_key(&(x - 1, y)) {
                        let thing_left = map.get(&(x - 1, y)).unwrap();
                        if thing_left.value != None {
                            let new_val = 10 * thing_left.value.unwrap() + thing_value.unwrap();
                            map.remove(&(x - 1, y));
                            map.get_mut(&(x, y)).unwrap().value = Some(new_val);
                        }
                    }
                }
            }
        }
    }

}

// I have no fucking clue why this is a HashMap and not a straight Vec<Vec<>>...
// I guess I am expecting a part-2 twist.

fn part1(data: &str) -> u32 {
    let mut map: HashMap<(i64, i64), Thing> = HashMap::new();
    data.split('\n')
        .enumerate()
        .map(|(y, s)| {
            for (x, c) in s.chars().enumerate() {
                if c != '.' {
                    map.insert((x as i64, y as i64), Thing::new(c));
                }
            }
        })
        .for_each(drop);

    // print_map(&map);

    merge_map(&mut map);
    // print_map_val(&map);

    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let xmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let ymin = *map.keys().map(|(_, y)| y).min().unwrap();
    let mut total: u32 = 0;
    let mut cur_val: u32 = 0;

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&(x, y)) {
                let thing_value = map.get(&(x, y)).unwrap().value;
                if thing_value != None {
                    if is_partnumber(&map, x, y) {
                        // println!(
                        //     "{},{} is part number {}",
                        //     x,
                        //     y,
                        //     map.get(&(x, y)).unwrap().value.unwrap()
                        // );
                        total += thing_value.unwrap();
                    }
                }
            }
        }
    }

    println!("Total: {}", total);

    total
}
fn part2(data: &str) -> u32 {
    888
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 525911);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 888);
    println!("Completed in {} us", now.elapsed().as_micros());
}
