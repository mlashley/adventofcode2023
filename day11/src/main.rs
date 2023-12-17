use log::{debug, info};
use std::collections::HashSet;
use std::env;
use std::time::Instant;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            2
        ) == 374
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            10
        ) == 1030
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            100
        ) == 8410
    );
}

fn print_universe(galaxies: &HashSet<(usize, usize)>) {
    let x_max = galaxies.iter().map(|g| g.0).max().unwrap() + 1;
    if x_max > 4000 {
        debug!("You are insane - refusing to print giant map, your 40 inch curved widescreen wang is not /that/ huge");
        return;
    }
    let y_max = galaxies.iter().map(|g| g.1).max().unwrap() + 1;
    let mut s = String::new();
    for y in 0..y_max {
        for x in 0..x_max {
            if galaxies.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    debug!("Universe:\n{}", s);
}

// Function to expand the universe based on the given rules
fn expand_universe(galaxies: &mut HashSet<(usize, usize)>, universe: Vec<Vec<char>>, time: usize) {
    // Identify rows without galaxies and insert new rows above them
    let mut new_galaxies = galaxies.clone();
    let mut added = 0;

    let add_time = time -1 ; // We are replace 100 for 1  == add 99.

    for (y, row) in universe.iter().enumerate() {
        if !row.contains(&'#') {
            debug!("New row at {}", y);
            for galaxy in new_galaxies.clone().iter() {
                if galaxy.1 > y + added {
                    new_galaxies.remove(galaxy);
                    new_galaxies.insert((galaxy.0, galaxy.1 + add_time));
                }
            }
            added += add_time;
        }
    }

    let mut v = Vec::from_iter(new_galaxies.clone());
    v.sort();
    debug!("new_galaxies:   {:?}", v);

    // Identify columns without galaxies and insert new columns to the left of them

    let cols = universe[0].len();
    let mut added = 0;
    for col in 0..cols {
        if !universe.iter().any(|row| row[col] == '#') {
            debug!("New col at {}", col);
            for galaxy in new_galaxies.clone().iter() {
                if galaxy.0 > col + added {
                    new_galaxies.remove(galaxy);
                    new_galaxies.insert((galaxy.0 + add_time, galaxy.1));
                }
            }
            added += add_time;
            let mut v = Vec::from_iter(new_galaxies.clone());
            v.sort();
            debug!("newer_galaxies: {:?}", v);
        }
    }
    debug!("newer_galaxies: {:?}", new_galaxies);
    print_universe(&new_galaxies);

    *galaxies = new_galaxies;
}
// Function to find the shortest path length between two galaxies
fn shortest_path_length(start: &(usize, usize), end: &(usize, usize)) -> i64 {
    (end.1 as i64 - start.1 as i64).abs() + (end.0 as i64 - start.0 as i64).abs()
}

// Function to calculate the sum of shortest path lengths between all pairs of galaxies
fn sum_of_shortest_paths(galaxies: HashSet<(usize, usize)>) -> i64 {
    let mut total_length = 0;
    let mut seen = HashSet::new();

    for galaxy_from in &galaxies {
        for galaxy_to in &galaxies {
            if galaxy_from != galaxy_to
                && !seen.contains(&(galaxy_from, galaxy_to))
                && !seen.contains(&(galaxy_to, galaxy_from))
            {
                seen.insert((galaxy_from, galaxy_to));
                let length = shortest_path_length(galaxy_from, galaxy_to);
                debug!(
                    "{},{} => {},{} len: {} tot: {}",
                    galaxy_from.0, galaxy_from.1, galaxy_to.0, galaxy_to.1, length, total_length
                );
                total_length += length;
            }
        }
    }
    total_length
}

fn part1(data: &str, time: usize) -> i64 {
    let universe: Vec<_> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut galaxies = HashSet::new();
    data.split('\n')
        .filter(|y| !y.is_empty())
        .enumerate()
        .map(|(y, x)| {
            for (xx, c) in x.chars().enumerate() {
                if c == '#' {
                    galaxies.insert((xx, y));
                }
            }
        })
        .for_each(drop);

    debug!("We have {} galaxies", galaxies.len());

    debug!("{:?}", galaxies);

    expand_universe(&mut galaxies, universe, time);

    debug!("{:?}", galaxies);

    let result = sum_of_shortest_paths(galaxies);

    debug!("Sum of shortest path lengths: {}", result);
    result
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), 2);
    info!("Part1: {}", p1);
    assert!(p1 == 10292708);
    let p2 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        1000000,
    );
    info!("Part2: {}", p2);
    assert!(p2 == 790194712336);
    info!("Completed in {} us", now.elapsed().as_micros());
}
