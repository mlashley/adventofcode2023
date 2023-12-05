use parse_display::{Display, FromStr};
use std::{cmp::max, time::Instant};

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"Game {id}: {results}")]
struct Game {
    id: i32,
    results: String,
}

#[derive(Display, FromStr, Debug)]
#[display(r"{val} {color}")]
struct Result {
    val: i32,
    color: String,
}

impl Game {
    fn is_possible(&self, r_pos: i32, g_pos: i32, b_pos: i32) -> bool {
        let mut r_max = 0;
        let mut g_max = 0;
        let mut b_max = 0;

        let results = self
            .results
            .split(';')
            .map(|res| {
                res.trim()
                    .split(',')
                    .map(|x| {
                        let q = x.trim().parse::<Result>().unwrap();
                        match q.color.as_str() {
                            "red" => r_max = max(r_max, q.val),
                            "green" => g_max = max(g_max, q.val),
                            "blue" => b_max = max(b_max, q.val),
                            &_ => todo!(),
                        }
                        q
                    })
                    .collect::<Vec<Result>>()
            })
            .collect::<Vec<_>>();

        // println!("R: {:#?}",results);
        // println!("R,G,Bmax: {},{},{}",r_max,g_max,b_max);
        r_max <= r_pos && g_max <= g_pos && b_max <= b_pos
    }
}

fn test() {
    let g1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        .parse::<Game>()
        .unwrap();

    debug_assert!(g1.clone().is_possible(4, 2, 6) == true);
    debug_assert!(g1.clone().is_possible(2, 2, 6) == false);
    debug_assert!(g1.clone().is_possible(4, 1, 6) == false);
    debug_assert!(g1.clone().is_possible(4, 2, 5) == false);

    debug_assert!(g1.id == 1);
    debug_assert!(g1.results == "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 8
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

fn part1(data: &str) -> i32 {
    let games: Vec<Game> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Game>().unwrap())
        .collect();

    games
        .iter()
        .map(|g| if g.is_possible(12, 13, 14) { g.id } else { 0 })
        .sum()
}
fn part2(data: &str) -> usize {
    888
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 2237);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 888);
    println!("Completed in {} us", now.elapsed().as_micros());
}
