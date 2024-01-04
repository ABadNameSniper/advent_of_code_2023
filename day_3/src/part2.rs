use std::fs;
use std::cmp::min;
use std::iter::once;

fn main() {
    let mut char_grid: Vec<Vec<char>> = fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|l| l.chars().collect::<Vec<char>>())
    .collect();    

    let mut ratios: Vec<u32> = Vec::new();

    let mut y = 0; 
    let height = char_grid.len();
    while y < height {
        let width = char_grid[y].len();
        let mut gear_location = None;
        let mut x = 0;
        while x < width {
            let c = char_grid[y][x];
            //start of num
            if c.is_ascii_digit() {
                let num_index = x;
                //verbose, but nothing is unecessarily checked I guess
                if x > 0 {
                    for i in y.checked_sub(1).unwrap_or(0)..min(y + 2, height) {
                        if char_grid[i][x - 1] == '*' {
                            gear_location = Some((i, x - 1));
                            break;
                        }
                    }
                }
                while x < width && char_grid[y][x].is_ascii_digit() {
                    if gear_location.eq(&None) {
                        if y > 0 && char_grid[y - 1][x] == '*' {
                            gear_location = Some((y - 1, x));
                        } else if y < char_grid.len() - 1 && char_grid[y + 1][x] == '*' {
                            gear_location = Some((y + 1, x));
                        }
                    }
                    x += 1;
                }
                if x < width {
                    for i in y.checked_sub(1).unwrap_or(0)..min(y + 2, height) {
                        if char_grid[i][x] == '*' {
                            gear_location = Some((i, x));
                            break;
                        }
                    }
                }
                if gear_location.is_some() {
                    let primary_ratio: u32 = replace_nums(&mut char_grid, y, num_index, x);
                    let mut secondary_ratio: Option<u32> = None;
                    let (y, x) = gear_location.unwrap();
                    for i in y.checked_sub(1).unwrap_or(0)..min(y + 2, height) {
                        let mut j = x.checked_sub(1).unwrap_or(0);
                        while j < min(x + 2, width) {
                            if char_grid[i][j].is_ascii_digit() {
                                //find first digit in number
                                let mut back = j;
                                while back > 0 && char_grid[i][back - 1].is_ascii_digit() {
                                    back -= 1;
                                }
                                let secondary_index = back;
                                //increment to last digit
                                while j < width && char_grid[i][j].is_ascii_digit() {
                                    j += 1;
                                }
                                secondary_ratio = Some(replace_nums(&mut char_grid, i, secondary_index, j));
                                break;
                            }
                            j += 1;
                        }
                        //assuming max of two nums together! break here
                        if secondary_ratio.is_some() {
                            break;
                        }
                    }
                    if secondary_ratio.is_some() {
                        ratios.push(primary_ratio * secondary_ratio.unwrap());
                    }
                    gear_location = None;
                }
            }
            x += 1;
        }
        y += 1;
    }
    println!("{:?}", ratios);
    println!("{}", ratios.iter().sum::<u32>());

    println!("{}", String::from_iter(char_grid.iter().map(|line| String::from_iter(line.iter().chain(once(&'\n'))))));
}

fn replace_nums(grid: &mut Vec<Vec<char>>, line: usize, start: usize, end: usize) -> u32 {
    grid[line][start..end]
        .iter_mut()
        .map(|char| std::mem::replace(char, '.'))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}