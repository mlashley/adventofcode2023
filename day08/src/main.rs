// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use log::{debug, error, info};
use parse_display::{Display, FromStr};
use std::env;
use std::time::Instant;
use std::collections::HashMap;
use num::Integer;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 2
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample2.txt")
                .unwrap()
                .as_str()
        ) == 6
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample3.txt")
                .unwrap()
                .as_str()
        ) == 6
    );
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"{from} = ({left}, {right})")]
struct Node {
    from: String,
    left: String,
    right: String,
}

fn part1(data: &str) -> usize {
    let chunks: Vec<_> = data.split("\n\n").filter(|y| !y.is_empty()).collect();
    let directions = chunks[0].chars().cycle(); // repeating iterator
    
    // Build the network
    let mut node_hashmap: HashMap<String, Node> = HashMap::new();
    chunks[1]
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Node>().unwrap())
        .map(|x| { node_hashmap.insert(x.from.clone(),x.clone()) })
        .for_each(drop);
    debug!("MAP: {:?}", node_hashmap);

    // Walk it
    let mut current = "AAA";
    let mut count = 0;
    for d in directions {
        current = match d {
            'L' => &node_hashmap.get(current).unwrap().left,
            'R' => &node_hashmap.get(current).unwrap().right,
            _   => { error!("Fuuu"); "Fuu" },
        };      
        count +=1;
        if current == "ZZZ" { break; }
    }
    count
}
fn part2(data: &str) -> u64 {
    
    let chunks: Vec<_> = data.split("\n\n").filter(|y| !y.is_empty()).collect();
    let directions = chunks[0].chars().cycle(); // repeating iterator
    
    // Build the network
    let mut node_hashmap: HashMap<String, Node> = HashMap::new();
    let mut curr_vec:Vec<_> = Vec::new();
    chunks[1]
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Node>().unwrap())
        .map(|x| { node_hashmap.insert(x.from.clone(),x.clone()) })
        .for_each(drop);

    for k in  node_hashmap.keys() {
        if k.ends_with('A') { 
            curr_vec.append(&mut vec![k.to_string()]);
        };
    }

    debug!("CURR MAP: {:?}", curr_vec);

    // Walk it
        
    let mut count = 0;
    let mut last_seen = vec![0; curr_vec.len()];
    let mut periods = vec![0; curr_vec.len()];
    
    for d in directions {
        let mut next_vec:Vec<_> = Vec::new();
        for (i,current) in curr_vec.into_iter().enumerate() {
            let next = match d {
                'L' => &node_hashmap.get(&current).unwrap().left,
                'R' => &node_hashmap.get(&current).unwrap().right,
                _   => todo!(),
            };      
            next_vec.push( next.to_string());
            if next.ends_with('Z') { 
                if periods[i] == 0 && last_seen[i] == 0 {
                    last_seen[i] = count;
                } else if periods[i] == 0 {                                 
                    debug!("[{}] {}",i,count-last_seen[i]);
                    periods[i] = count-last_seen[i];
                    last_seen[i] = count;
                }
                if periods.clone().into_iter().filter(|x|*x==0).count() == 0 {
                    break;
                }
            }
        }
        count +=1;

        // Billions of CPU cycles later we might trigger this...
        if next_vec.iter().filter(|x| x.ends_with('Z')).count() == next_vec.len() { break; }
        curr_vec = next_vec;

        if count % 10000000 == 0 {
        debug!("Count: {}",count);
        debug!("CURR MAP: {:?}", curr_vec);
        }
    }
    // We recall this from another year - find the periods of each one, and then take the product of their lcm's
    let mut lcm_result:u64 = periods[0];
    for p in periods {
        lcm_result = p.lcm(&lcm_result);

    }  
    lcm_result
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
    assert!(p1 == 22199);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 13334102464297);
    info!("Completed in {} us", now.elapsed().as_micros());
}
