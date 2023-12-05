use std::time::Instant;

fn test() {
    let a = "Malc1thing2three3are";
    let first: Option<u32> = a.chars().find_map(|c| c.to_digit(10));
    let last: Option<u32> = a.chars().rev().find_map(|c| c.to_digit(10));
    debug_assert!(first.unwrap() * 10 + last.unwrap() == 13);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 142
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

fn part1(data: &str) -> u32 {
    let mut total = 0;
    data.trim().split('\n').for_each(|f| {
        let first: Option<u32> = f.chars().find_map(|c| c.to_digit(10));
        let last: Option<u32> = f.chars().rev().find_map(|c| c.to_digit(10));
        // print!("Ans: {} \n",first.unwrap()*10 + last.unwrap());
        total += first.unwrap() * 10 + last.unwrap();
    });

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
    assert!(p1 == 54953);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 888);
    println!("Completed in {} us", now.elapsed().as_micros());
}
