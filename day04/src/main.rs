use parse_display::{Display, FromStr};
use std::time::Instant;

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"Card {id}: {winners} | {mine}")]
#[from_str(regex = "Card +(?<id>[0-9]+): (?<winners>[0-9 ]+)\\|(?<mine>[0-9 ]+)")]

struct Game {
    id: i32,
    winners: String,
    mine: String,
}

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 13
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 30
    );
}

fn score(g: &Game, is_part2: bool) -> u32 {
    // println!("Game {} Scoring winners {} for {}",g.id,g.winners,g.mine );

    let my_numbers: Vec<u32> = g
        .mine
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let win_numbers: Vec<u32> = g
        .winners
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut score = 0;
    for win in win_numbers {
        if my_numbers.contains(&win) {
            // println!("w:{}",win);
            score += 1
        }
    }

    if score == 0 {
        return 0;
    }
    if is_part2 {
        return score;
    }
    return 2_u32.pow(score - 1);
}

fn part1(data: &str) -> u32 {
    data.split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Game>().unwrap())
        .map(|x| score(&x, false))
        .sum()
}
fn part2(data: &str) -> u32 {
    let len = data.split('\n').filter(|y| !y.is_empty()).count();
    let mut cards = vec![1; len];

    data.split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Game>().unwrap())
        .enumerate()
        .map(|(i, x)| {
            let s = usize::try_from(score(&x, true)).unwrap();
            // println!("{} scored {}",i,s);
            for id in i + 1..i + s + 1 {
                cards[id] += cards[i];
            }
        })
        .for_each(drop);
    // println!("{:?}", cards);
    cards.into_iter().sum()
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 20407);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 23806951);
    println!("Completed in {} us", now.elapsed().as_micros());
}
