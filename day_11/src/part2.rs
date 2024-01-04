use std::{fs, cmp::{min, max}};

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let grid = input
    .lines()
    .map(|line| {
        line.as_bytes().to_vec()
    })
    .collect::<Vec<_>>();

    let expansions_y = expand(&grid);
    println!("Expansions y: {}", find_distance(&expansions_y, 0, expansions_y.len()));

    let mut rotation: Vec<Vec<u8>> = vec![Vec::new(); grid[0].len()];
    for line in &grid {
        for (byte_index, byte) in line.as_slice().iter().enumerate() {
            rotation[byte_index].push(*byte);
        }
    }

    let expansions_x = expand(&rotation);
    println!("Expansions x: {}", find_distance(&expansions_x, 0, expansions_x.len()));


    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            if *byte == b'#' {
                // remember that they're rotated
                galaxies.push((x, y));
            }
        }
    }

    let mut sum = 0;
    let mut total = 0;
    for first_galaxy in &galaxies {
        for index in total..galaxies.len() {
            let second_galaxy = galaxies[index];
            sum +=  find_distance(&expansions_x, first_galaxy.0, second_galaxy.0) + 
                    find_distance(&expansions_y, first_galaxy.1, second_galaxy.1);

        }
        total += 1;
    }

    println!("{sum}");

}

fn expand(grid: &Vec<Vec<u8>>) -> Vec<bool> {
    let mut expansions = vec![false; grid.len()];
    for (index, line) in grid.iter().enumerate() {
        if line.iter().all(|byte| *byte == b'.') {
            expansions[index] = true;
        }
    }
    expansions
}

const EXPANSION_SCALE: usize = 1_000_000;

fn find_distance(expansions: &Vec<bool>, point_a: usize, point_b: usize) -> usize {
    expansions[
        min(point_a, point_b)..
        max(point_a, point_b)
    ]
    .iter()
    .map(|expanded| {
        match *expanded {
            true => EXPANSION_SCALE,
            false => 1
        }
    })
    .sum()
}