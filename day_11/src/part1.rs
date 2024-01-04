use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let grid = input
    .lines()
    .map(|line| {
        line.as_bytes().to_vec()
    })
    .collect::<Vec<_>>();
    println!("{}\n", grid.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n"));

    let expanded = expand(grid);
    println!("{}\n", expanded.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n"));

    let mut rotation: Vec<Vec<u8>> = vec![Vec::new(); expanded[0].len()];
    for line in expanded {
        for (byte_index, byte) in line.as_slice().iter().enumerate() {
            rotation[byte_index].push(*byte);
        }
    }
    println!("{}\n", rotation.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n"));

    let enhanced = expand(rotation);
    println!("{}\n", enhanced.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n"));


    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, line) in enhanced.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            if *byte == b'#' {
                galaxies.push((y, x));
            }
        }
    }

    let mut sum = 0;
    let mut total = 0;
    for (g1y, g1x) in galaxies.clone() {
        for index in total..galaxies.len() {
            sum += 
            (g1y as isize - galaxies[index].0 as isize).abs() 
            + (g1x as isize - galaxies[index].1 as isize).abs() 
        }
        total += 1
    }

    //I tried to speedrun this (37 minutes) ((i'm still learning rust, ok?))
    //note: minor fixes beyond that
    println!("{sum}");

    println!("{}", true as u8);

}

fn expand(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut expanded = grid.clone();
    for (index, line) in grid.iter().rev().enumerate() {
        if line.iter().all(|byte| *byte == b'.') {
            println!("{index}");
            expanded.insert(grid.len() - index, vec![b'.'; grid[0].len()]);
        }
    }
    expanded
}