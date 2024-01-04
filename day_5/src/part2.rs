use std::{fs, cmp::min};
use std::time::Instant;


fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let mut sections = input
    .split(":");

    let seeds: Vec<u64> = sections
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse::<u64>().ok())
    .collect();

    let maps: Vec<Vec<MapLine>>= sections.map(|section| 
        section
        .lines()
        .filter_map(|l| {
            let l = l
            .split_whitespace()
            .filter_map(|s| {
                return s.parse::<u64>().ok()
            })
            .collect::<Vec<u64>>();

            if l.is_empty() {
                None
            } else {
                Some(MapLine {
                    destination: l[0], 
                    start: l[1], 
                    end: l[2] + l[1]
                })
            }
        })
        .collect::<Vec<MapLine>>()
    )
    .collect();

    let mut minimum = u64::MAX;

    for seed_start in (0..seeds.len()).step_by(2) {
        for seed in seeds[seed_start]..(seeds[seed_start] + seeds[seed_start+1]) {
            let mut seed_val = seed;
            for map in maps.iter() {
                for map_line in map {
                    if 
                        seed_val >= map_line.start &&
                        seed_val < map_line.end
                    {
                        seed_val = map_line.destination + (seed_val - map_line.start);
                        break;
                    }
                }
            }
            minimum = min(minimum, seed_val);
        }
    }

    println!("{}", minimum);
    println!("{}, {}", minimum, start.elapsed().as_secs());

}

#[derive(Debug)]
struct MapLine {
    destination: u64,
    start: u64,
    end: u64
}