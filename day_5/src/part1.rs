use std::fs;

fn main() {
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
                    source: l[1], 
                    range: l[2]
                })
            }
        })
        .collect::<Vec<MapLine>>()
    )
    .collect();

    let mut locations: Vec<u64> = Vec::new();

    for seed in &seeds {
        let mut seed_val = *seed;
        for map in maps.iter() {
            for map_line in map {
                if 
                    seed_val >= map_line.source &&
                    seed_val < (map_line.source + map_line.range)
                {
                    seed_val = map_line.destination + (seed_val - map_line.source);
                    break;
                }
            }
        }
        locations.push(seed_val);
    }

    println!("{}", locations.iter().min().unwrap());
}

#[derive(Debug)]
struct MapLine {
    destination: u64,
    source: u64,
    range: u64
}