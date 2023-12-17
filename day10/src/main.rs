use std::{time::Instant, collections::HashMap};
use std::env;
use log::{debug, error, info, warn,};

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(), false,
        ) == 4
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample2.txt")
                .unwrap()
                .as_str(), false,
        ) == 8
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample3.txt")
                .unwrap()
                .as_str(), true,
        ) == 10
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

// fn printLoop (pipe_map: &Vec<Vec<char>>, visited_map: &HashMap::<(usize,usize),bool>) {
//     let mut s = String::new();
//     s.push('\n');
//     for (y,row) in pipe_map.iter().enumerate() {
//         for (x,c) in row.iter().enumerate() {
//             if visited_map.contains_key(&(x,y)) {
//                 s.push(*c);
//             } else {
//                 s.push(' ');
//             }
//         }
//         s.push('\n')
//     }
//     info!("{}",s);
// }

fn fill_loop (pipe_map: &Vec<Vec<char>>, visited_map: &HashMap::<(usize,usize),bool>) -> usize {

    let mut fill_map:Vec<Vec<char>> = Vec::new();

    // fillmap is going to be 3x the size so we can render pipes

    for (y,row) in pipe_map.iter().enumerate() {
        let mut fill_row1:Vec<char> = Vec::new();
        let mut fill_row2:Vec<char> = Vec::new();
        let mut fill_row3:Vec<char> = Vec::new();
        for (x,c) in row.iter().enumerate() {
            if visited_map.contains_key(&(x,y)) {

                let mut r1 = match *c {
                    '|' => vec![' ','X',' '],
                    '-' => vec![' ',' ',' '],
                    'L' => vec![' ','X',' '],
                    'J' => vec![' ','X',' '],
                    '7' => vec![' ',' ',' '],
                    'F' => vec![' ',' ',' '],
                    _   => vec!['E','R','R'],
                };
                fill_row1.append(&mut r1);

                let mut r2 = match *c {
                    '|' => vec![' ','X',' '],
                    '-' => vec!['X','X','X'],
                    'L' => vec![' ','X','X'],
                    'J' => vec!['X','X',' '],
                    '7' => vec!['X','X',' '],
                    'F' => vec![' ','X'        ,'X'],
                    _   => vec!['E','R','R'],
                };
                fill_row2.append(&mut r2);
                let mut r3 = match *c {
                    '|' => vec![' ','X',' '],
                    '-' => vec![' ',' ',' '],
                    'L' => vec![' ',' ',' '],
                    'J' => vec![' ',' ',' '],
                    '7' => vec![' ','X',' '],
                    'F' => vec![' ','X',' '],
                    _   => vec!['E','R','R'],
                };
                fill_row3.append(&mut r3);
            } else {
                fill_row1.append(&mut vec![' ',' ',' '],);
                fill_row2.append(&mut vec![' ',' ',' '],);
                fill_row3.append(&mut vec![' ',' ',' '],);
            }
        }
        fill_map.push(fill_row1);
        fill_map.push(fill_row2);
        fill_map.push(fill_row3);
    }

    // Now assume we can fill from (0,0), because we padded.
    let mut q:Vec<(usize,usize)> = Vec::new();
    q.push((0,0));

    let ymax = fill_map.len()-1;
    let xmax = fill_map[0].len()-1;

    // Flood fill
    while let Some(n) = q.pop() {
        if fill_map[n.1][n.0] == ' ' {
            fill_map[n.1][n.0] = '.';
            if n.0 > 0 { q.push((n.0-1,n.1))}
            if n.0 < xmax { q.push((n.0+1,n.1))}
            if n.1 > 0 { q.push((n.0,n.1-1))}
            if n.1 < ymax { q.push((n.0,n.1+1))}
        }
    }

    // Display
    for row in &fill_map {
        for c in row {
            print!("{}",c);
        }
        println!();
    }
    let mut count = 0;
    // For any shape - only need to check the middle row is empty to be sure it is blank
    for row in fill_map[1..].iter().step_by(3) {
        for c in row.chunks(3) {
            let s = c.iter().collect::<String>();
            if s == "   " { count += 1}
        }
    }
    count
}

fn part1(data: &str, is_part2: bool) -> usize {
    let pipe_map:Vec<_> = data.split('\n')
    .filter(|y| !y.is_empty())
    .map(|x| x.chars().collect::<Vec<char>>())
    .collect();

    let mut visited = HashMap::<(usize,usize),bool>::new();

    // Find Start 'S'
    let mut s_pos = (0,0);
    for (y,row) in pipe_map.iter().enumerate() {
        for (x,c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos=(x,y);
                visited.insert(s_pos, true);
                break;
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
            visited.insert(current_pos, true);
            break;
        }
    }

    let mut length = 1;

    while current_pos != s_pos {
        if let Some(res) = connects_us(&pipe_map, current_pos, current_dir) { 
            debug!("{:?} {:?} {:?}",current_pos, current_dir,res);
            current_pos = (res.0,res.1);
            current_dir = res.2;
            visited.insert(current_pos, true);
            length += 1;
            if current_dir == '*' {
                debug!("Home my homie after {}",length);
                break; // We reached back to the start.
            }
        } else {
            error!("Fuuu");
        }
    }   
    debug!("Visited: {:?}",visited);

    if is_part2 {
        return fill_loop(&pipe_map, &visited)
    }
    length/2 
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(), false
    );
    info!("Part1: {}", p1);
    assert!(p1 == 6690);
    let p2 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(),true);
    info!("Part2: {}", p2);
    assert!(p2 == 525);
    info!("Completed in {} us", now.elapsed().as_micros());
}
