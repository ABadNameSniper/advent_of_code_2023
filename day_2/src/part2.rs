use std::fs;
use regex::Regex;

fn main() {
    const COLOR_AMOUNT: usize = 3;
    let input = fs::read_to_string("input.txt").unwrap();
    let set_regex = Regex::new(r"(?: \d+ \w+,?)+").unwrap();
    let colors = ["red", "green", "blue"];
    let color_regexes = colors.map(|c: &str| Regex::new(format!(r"(\d+) {}", c).as_str()).unwrap());

    let games: Vec<_> = input
    .lines()
    .map(
        |line| 
        set_regex.find_iter(line).map(
            |mat| {
            let mut blocks: [u8; COLOR_AMOUNT] = [0; COLOR_AMOUNT];
            for color in 0..blocks.len() {
                blocks[color] = match color_regexes[color].captures(mat.as_str()) {
                    None => 0,
                    Some(cap) => cap[1].parse().unwrap()
                }
            }
            blocks
        })
        .collect::<Vec<[u8; COLOR_AMOUNT]>>()
    )
    .collect();

    let mut minimums: Vec<[u8; COLOR_AMOUNT]> = Vec::new(); 

    for game_id in 0..games.len() {
        minimums.push([0; COLOR_AMOUNT]);
        for set in &games[game_id] {
            for index in 0..colors.len() {
                if set[index] >  minimums[game_id][index] {
                    minimums[game_id][index] = set[index]
                }
            }
        }
    }

    let mut color_powers: Vec<u32> = Vec::new();
    for game in &minimums {
        let mut color_power = game[0] as u32;
        for color in &game[1..game.len()] {
            color_power *= *color as u32;
        }
        color_powers.push(color_power);
    }

    let total: u32 = color_powers.iter().sum();

    println!("{:#?}", total);
}
