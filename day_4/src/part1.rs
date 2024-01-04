use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games = input.lines()
    .map(|line| {
        let mut line = line.split(" | ");
        let first_half = line.next().unwrap();
        let win_nums: Vec<u32> = first_half
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let own_nums: Vec<u32> = line
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        [win_nums, own_nums]
    });

    let points: Vec<usize> = games.map(|game| {
        let mut points = 1;
        game[0].iter().for_each(|win_num| {
            game[1].iter().for_each(|own_num| {
                if own_num == win_num {
                    points *= 2;
                }
            })
        });
        points >> 1
    }).collect();

    println!("{:?}", points.iter().sum::<usize>());
}
