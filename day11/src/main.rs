use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::HashSet;
use std::env;
use std::time::Instant;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 374
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

// Function to expand the universe based on the given rules
fn expand_universe(universe: &mut Vec<Vec<char>>) {
    let cols = universe[0].len();

    // Identify rows without galaxies and insert new rows above them
    let mut new_universe = Vec::new();
    for row in universe.iter() {
        if !row.contains(&'#') {
            new_universe.push(vec!['.'; cols]);
        }
        new_universe.push(row.clone());
    }

    debug!("New rows >>>>");
    for row in &new_universe {
        debug!("{:?}", row);
    }
    debug!("<<<< New rows");

    // Identify columns without galaxies and insert new columns to the left of them
    let mut expanded_universe = new_universe.clone();
    let mut added = 0;
    for col in 0..cols {
        if !new_universe.iter().any(|row| row[col] == '#') {
            debug!("New col at {} ({})", col, col + added);
            for row in expanded_universe.iter_mut() {
                row.push('.');
                for c in (col + added..row.len()).rev() {
                    row[c] = row[c - 1];
                }
                row[col + added] = '.';
            }
            added += 1;

            debug!("New cols >>>>");
            for row in &expanded_universe {
                debug!("{:?}", row);
            }
            debug!("<<<< New cols");
        }
    }

    *universe = expanded_universe;
}
// Function to find the shortest path length between two galaxies
fn shortest_path_length(start: (usize, usize), end: (usize, usize)) -> i64 {
    (end.1 as i64 - start.1 as i64).abs() + (end.0 as i64 - start.0 as i64).abs()
}

// Function to calculate the sum of shortest path lengths between all pairs of galaxies
fn sum_of_shortest_paths(universe: &Vec<Vec<char>>) -> i64 {
    let mut total_length = 0;
    let mut seen = HashSet::new();

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == '#' {
                for x in 0..universe.len() {
                    for y in 0..universe[0].len() {
                        if universe[x][y] == '#'
                            && !seen.contains(&(i, j, x, y)) & !seen.contains(&(x, y, i, j))
                        {
                            let length = shortest_path_length((i, j), (x, y));
                            seen.insert((i, j, x, y));
                            debug!("{},{} => {},{} tot: {}", i, j, x, y, total_length);
                            total_length += length;
                        }
                    }
                }
            }
        }
    }

    total_length
}

fn part1(data: &str) -> i64 {
    let mut universe: Vec<_> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    expand_universe(&mut universe);

    let result = sum_of_shortest_paths(&universe);

    debug!("Sum of shortest path lengths: {}", result);
    result
}
fn part2(data: &str) -> i64 {
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
    assert!(p1 == 10292708);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
