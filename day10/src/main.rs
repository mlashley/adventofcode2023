// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::time::Instant;
use std::env;
use log::{debug, error, info, log_enabled, warn, Level};

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 4
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample2.txt")
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

// Look at the pipe in the given direction to see if it connects back to us, if it does - provide the exit-direction from that pipe
fn connects_us (pipe_map: &Vec<Vec<char>>, pos: (usize,usize), dir: char) -> Option<(usize,usize,char)> {
    let ymax = pipe_map.len();
    let xmax = pipe_map[0].len();

    match dir {
        'N' => { 
            if pos.1 == 0 { return None };  // Flat earth...
            let y=pos.1-1; let x = pos.0;
            debug!("Looking {} to {},{} at {}", dir,x,y,&pipe_map[y][x]);
            match &pipe_map[y][x] {
                '|' => Some((x,y, 'N')),
                '-' => None,
                'L' => None,
                'J' => None,
                '7' => Some((x,y, 'W')),
                'F' => Some((x,y, 'E')),
                '.' => None,
                'S' => Some((x,y,'*')),
                _ => { warn!("Foo Fighters to the North"); None },
            }
        },
        'E' => {
            if pos.0 == xmax { return None };
            let y=pos.1; let x = pos.0+1;
            debug!("Looking {} to {},{} at {}", dir,x,y,&pipe_map[y][x]);
            match &pipe_map[y][x] {
                '|' => None,
                '-' => Some((x,y, 'E')),
                'L' => None,
                'J' => Some((x,y, 'N')),
                '7' => Some((x,y, 'S')),
                'F' => None,
                '.' => None,
                'S' => Some((x,y,'*')),
                _ => { warn!("Foo Fighters to the East {} ", &pipe_map[y][x] ); None },
            }
        },
        'S' =>{
            if pos.1 == ymax { return None };        
            let y=pos.1+1; let x = pos.0;  
            debug!("Looking {} to {},{} at {}", dir,x,y,&pipe_map[y][x]);
            match &pipe_map[y][x] {
                '|' => Some((x,y, 'S')),
                '-' => None,
                'L' => Some((x,y, 'E')),
                'J' => Some((x,y, 'W')),
                '7' => None,
                'F' => None,
                '.' => None,
                'S' => Some((x,y,'*')),
                _ => { warn!("Foo Fighters to the South"); None },
            }
        }
        'W' => {
            if pos.0 == 0 { return None };
            let y=pos.1; let x = pos.0-1;
            debug!("Looking {} to {},{} at {}", dir,x,y,&pipe_map[y][x]);
            match &pipe_map[y][x] {
                '|' => None,
                '-' => Some((x,y, 'W')),
                'L' => Some((x,y, 'N')),
                'J' => None,
                '7' => None,
                'F' => Some((x,y, 'S')),
                '.' => None,
                'S' => Some((x,y,'*')),
                _ => { warn!("Foo Fighters to the West"); None },
            }
        },
        _ => { warn!("Running in the wrong direction?"); None },
    }
}


fn part1(data: &str) -> usize {
    let pipe_map:Vec<_> = data.split('\n')
    .filter(|y| !y.is_empty())
    .map(|x| x.chars().collect::<Vec<char>>())
    .collect();

    // Find Start 'S'
    let mut s_pos = (0,0);
    for (y,row) in pipe_map.iter().enumerate() {
        for (x,c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos=(x,y);
            }
        }
    }
    debug!("{} at {:?}",pipe_map[s_pos.1][s_pos.0], s_pos);

    // Find initial direction.
    let mut current_pos = (s_pos.0,s_pos.1);
    let mut current_dir = '*';
    
    for dir in ['N','S','E','W'] {
        if let Some(res) = connects_us(&pipe_map, current_pos, dir) {
            debug!("{}: {:?}",dir,res);
            current_pos = (res.0,res.1);
            current_dir = res.2;
            break;
        }
    }
    
    let mut length = 1;

    while current_pos != s_pos {
        if let Some(res) = connects_us(&pipe_map, current_pos, current_dir) { 
            debug!("{:?} {:?} {:?}",current_pos, current_dir,res);
            current_pos = (res.0,res.1);
            current_dir = res.2;
            length += 1;
            if current_dir == '*' {
                debug!("Home my homie after {}",length);
                break; // We reached back to the start.
            }
        } else {
            error!("Fuck");
        }
    }   
    length/2 
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
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str()
    );
    info!("Part1: {}", p1);
    assert!(p1 == 6690);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
