use std::fs;
use regex::Regex;

fn main() {
    const COLOR_AMOUNT: usize = 3;
    let rgb: [u8; COLOR_AMOUNT] = [12, 13, 14];
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

    println!("{:#?}", games);

    let mut possible_games = 0;
    for game_id in 0..games.len() {
        let mut possible = true;
        for set in &games[game_id] {
            for index in 0..colors.len() {
                if set[index] > rgb[index] {
                    possible = false;
                    break
                }
            }
        }
        if possible {
            possible_games += game_id + 1;
        }
    }

    println!("{}", possible_games);
}
