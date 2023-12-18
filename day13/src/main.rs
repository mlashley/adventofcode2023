use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::time::Instant;
use std::env;
use log::{debug, error, info, log_enabled, warn, Level};
use parse_display::{Display, FromStr};

fn test() { 
    debug!("SMUDGE: {}",smudge("#..\n...\n..#"));
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 405
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 400
    );
}

fn find_mirror(data: &str) -> usize {

    debug!("Hunting mirror in \n{}",data);

    let rows: Vec<_> = data.split('\n').filter(|y| !y.is_empty()).collect();
    let num_rows = rows.len();

    for center in 1..num_rows {
        // debug!("Try center: {}",center);
        let mut offset = 1;
        let mut found = true;
        while center>=offset && center+offset <= num_rows {
            debug!("{} <=> {}", center+(offset-1),center-offset);
            if rows[center+(offset-1)] != rows[center-offset] {
                found = false;
                break;
            }
            offset += 1;
        }
        debug!("ROWS({}): f:{} o:{}",center,found,offset);
        if found { return 100 *center} 
    }

    // Transpose and apply identical logic (save for *100)
    let mut cols:Vec<_> = Vec::new();
    let num_cols = rows[0].len();
    
    for i  in 0..num_cols {       
        cols.push(rows.iter().map(|x| x.chars().nth(i).unwrap()).collect::<String>());
    }

    for center in 1..num_cols {
        let mut offset = 1;
        let mut found = true;
        while center>=offset && center+offset <= num_cols {
            debug!("{} <=> {}", center+(offset-1),center-offset);
            if cols[center+(offset-1)] != cols[center-offset] {
                found = false;
                break;
            }
            offset += 1;
        }
        debug!("COLS({}): f:{} o:{}",center,found,offset);
        if found { return center}
    }
    
    warn!("Really no reflection??"); 
    0

}

fn smudge(data: &str)  -> String{
    let mut s = String::new();
    for i in 0..data.len() {       
        s += data.char_indices().map(|(j,c)| if j == i { 
            match c {
                '.' => '#',
                '#' => '.',
                '\n' => '\n',  // This is inefficient - we will generate some extra, unchanged patterns...
                _ => { warn!("Unexpected char in stream {}",c); '!' }
            }
        } else { c }
        ).collect::<String>().as_str();
        s+= "QWERTY";
    }
    s
}


fn part1(data: &str) -> usize {
    data
        .split("\n\n")
        .filter(|y| !y.is_empty())
        .map(|x| find_mirror(x))
        .sum()

}
fn part2(data: &str) -> usize {

    debug!("==== PART2 ====");
    let a = data
    .split("\n\n")
    .filter(|y| !y.is_empty())
    .map(|x| 
        { 
            let p1 = find_mirror(x);
            smudge(x)
            .split("QWERTY")
            .filter(|y| !y.is_empty())
            .map(|s| find_mirror(s))
            .map(|q| {info!("p1 {}  => {}",p1,q); q })
            .filter(|&r| r != 0)
            .filter(|&r| r != p1)
            
            .unique()
            
            .sum::<usize>()
        }

    )
    .sum();

    debug!("a={}",a);
    a

    // data
    // .split("\n\n")
    // .filter(|y| !y.is_empty())

    
    // 888
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str()
    );
    info!("Part1: {}", p1);
    assert!(p1 == 35210);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 > 23500);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
