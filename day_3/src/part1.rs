use std::fs;
use std::cmp::min;

fn main() {
    let char_grid: Vec<Vec<char>> = fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| l.chars().collect::<Vec<char>>())
    .collect();    

    let mut part_nums: Vec<u32> = Vec::new();

    for (y, line) in char_grid.iter().enumerate() {
        let mut part_number = false;
        let mut x = 0;
        while x < line.len() {
            let c = line[x];
            //start of num
            if c.is_ascii_digit() {
                let num_index = x;
                //verbose, but nothing is unecessarily checked I guess
                if x > 0 {
                    for i in y.checked_sub(1).unwrap_or(0)..min(y + 2, char_grid.len()) {
                        if is_symbol(char_grid[i][x - 1]) {
                            part_number = true;
                            break;
                        }
                    }
                }
                while x < line.len() && char_grid[y][x].is_ascii_digit() {
                    if !part_number &&
                        (y > 0  && is_symbol(char_grid[y - 1][x])) ||
                        (y < char_grid.len() - 1 && is_symbol(char_grid[y + 1][x]))
                    {
                        part_number = true;
                    }
                    x += 1;
                }
                if x < line.len() - 1 {
                    for i in y.checked_sub(1).unwrap_or(0)..min(y + 2, char_grid.len()) {
                        if is_symbol(char_grid[i][x]) {
                            part_number = true;
                            break;
                        }
                    }
                }
                if part_number {
                    part_nums.push(String::from_iter(&char_grid[y][num_index..x]).parse::<u32>().unwrap());
                    part_number = false;
                }
            }
            x += 1;
        }
        println!("{:?}", &part_nums);
        println!("{}", part_nums.iter().sum::<u32>());
        
    }
}

fn is_symbol(selection: char) -> bool {
    return !selection.is_ascii_digit() && selection != '.'
}