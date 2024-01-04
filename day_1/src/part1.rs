use std::fs;

fn main() {
    let mut total: u32 = 0;
    fs::read_to_string("input.txt").unwrap().lines().for_each(|line|
        total +=
        format!(
            "{}{}", 
            line.chars().find(|c: &char| c.is_ascii_digit()).unwrap(), 
            line.chars().rev().find(|c: &char| c.is_ascii_digit()).unwrap()
        ).parse::<u32>().unwrap()
    ); 
    println!("{}", total);
}