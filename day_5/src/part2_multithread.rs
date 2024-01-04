use std::{fs, cmp::min};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let mut sections = input
    .split(":");

    let seeds: Vec<u64> = sections
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse::<u64>().ok())
    .collect();

    let maps: Vec<Vec<MapLine>> = sections.map(|section| 
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

    //rearrange the ranges so they're smaller and more equal in length
    let mut sizes = Vec::new();

    let seeds: Vec<(u64, u64)> = seeds
    .iter()
    .enumerate()
    .step_by(2)
    .map(|(index, seed_start)| {
        let amount = seeds[index + 1];
        println!("Amount: {}, Beginning: {}, Index: {}", amount, seed_start, index);
        sizes.push(amount);
        (*seed_start, seed_start + amount)
    }).collect();

    let ideal_size = sizes.iter().min().unwrap(); //for my dataset, the smallest range works fine
    // ((*sizes.iter().min().unwrap() as f64) / 0.125 + 0.5) as u64;

    let mut seed_ranges = Vec::new();
    for (start, end) in seeds {
        let amount = end - start;//annoying, i just calculated all of these
        
        for chunk in 0..(amount / (ideal_size -1)) {
            println!("Amount: {amount}, Chunk: {chunk}");
            seed_ranges.push(
                (start + chunk * ideal_size,
                min(start + (chunk+1)*ideal_size, end))
            )
        }
    }

    println!("Ideal Size: {ideal_size}");

    //Iterate through each seed
    let minimum = u64::MAX;

    let result: u64 = seed_ranges
    .par_iter()
    .map(|(seed_start, seed_end)| {
        let mut minimum = minimum.clone();
        (*seed_start..*seed_end).for_each(|seed| {
            let mut seed_val = seed;
            for map in maps.iter(){
                for map_line in map {
                    if seed_val >= map_line.start && seed_val < map_line.end {
                        seed_val -= map_line.start - map_line.destination;
                        break;
                    }
                }
            }
            minimum = min(minimum, seed_val);
        });
        minimum
    })
    .min()
    .unwrap();
    
    println!("{}, {}", result, now.elapsed().as_secs());

}

#[derive(Debug)]
struct MapLine {
    destination: u64,
    start: u64,
    end: u64
}