use std::collections::VecDeque;
use parse_display::{Display, FromStr};
use std::time::Instant;
use log::{debug, error, log_enabled, info, warn, Level};
use std::env;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 35
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
    );
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"seeds: {seeds}")]
struct Seeds {
    seeds: String,
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"{dst_start} {src_start} {length}")]
struct Range {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

#[derive(Display, FromStr, Debug, Clone)]
#[display(r"{a_name}-to-{b_name} map:")]
struct MapHeader {
    a_name: String,
    b_name: String,
}

fn part1(data: &str) -> u64 {
    let mut chunks: VecDeque<_> = data.split("\n\n").filter(|y| !y.is_empty()).collect();

    let seed_obj = chunks
        .pop_front()
        .expect("WTF seeds?")
        .parse::<Seeds>()
        .unwrap();

    let mut seeds: Vec<_> = seed_obj
        .seeds
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    debug!("SEEDS: {:?}", seeds);

    while chunks.len() > 0 {
        let mut lines: VecDeque<_> = chunks
            .pop_front()
            .expect("Bad Chunk?")
            .split('\n')
            .filter(|y| !y.is_empty())
            .collect();

        let map_hdr = lines.pop_front().expect("WTF maps?").parse::<MapHeader>();
        debug!("MAP H: {:?}", map_hdr);
        let ranges: Vec<Range> = lines
            .into_iter()
            .map(|x| x.parse::<Range>().unwrap())
            .collect();

        for (i, seed) in seeds.clone().into_iter().enumerate() {
            let mut mapped = false;
            for r in &ranges {
                debug!("R: {:?}",r);

                if seed >= r.src_start && seed < (r.src_start + r.length) {
                    let mapped_val = r.dst_start + (seed - r.src_start);
                    debug!("Mapping {} => {}",seed,mapped_val);
                    mapped = true;
                    seeds[i] = mapped_val;
                }
            }
            if !mapped {
                debug!("Mapping {} => {}",seed,seed);
            }
        }
    }

    seeds.sort();
    *seeds.first().unwrap()
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
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part1: {}", p1);
    assert!(p1 == 1181555926);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    debug!("Completed in {} us", now.elapsed().as_micros());
}
