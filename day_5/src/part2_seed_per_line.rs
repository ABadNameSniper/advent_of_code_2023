use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sections = input
    .split(":");

    let seed_instructions: Vec<u64> = sections
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse::<u64>().ok())
    .collect();

    let mut seeds: Vec<u64> = Vec::new();
    for seed_start in (0..seed_instructions.len()).step_by(2) {
        seeds.extend(seed_instructions[seed_start]..(seed_instructions[seed_start] + seed_instructions[seed_start+1]));
    }

    let mut changed: Vec<bool> = vec![false; seeds.len()];

    sections.for_each(|section| 
        section
        .lines()
        .enumerate()
        .for_each(|(n, l)| {
            let l = l
            .split_whitespace()
            .filter_map(|s| {
                return s.parse::<u64>().ok()
            })
            .collect::<Vec<u64>>();

            if !l.is_empty() {
                seeds.iter_mut().enumerate().for_each(|(i, s)| 
                    if 
                        !changed[i]
                        || *s < l[1]
                        || *s >= l[1] + l[2]
                    {
                        changed[i] = true;
                        *s = l[0] + (*s - l[1]);
                    }
                );
            }
            changed = vec![false; seeds.len()];
        })
    );

    println!("{}", seeds.iter().min().unwrap());
}