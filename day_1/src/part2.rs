use std::fs;
use aho_corasick::AhoCorasick;

fn main() {
    let digits: Vec<String> = (0..10).map(|f| f.to_string())
    .chain(["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].map(|s| s.to_string())).collect();
    //Rust regex cannot be used due to consuming matches. oneight -> one, when it should return both one and eight.
    let ac = AhoCorasick::new(digits).unwrap();
    let mut total: u32 = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let numbers: Vec<u32> = ac
            .find_overlapping_iter(line)
            .map(|m| m.pattern().as_u32() % 10)
            .collect();
        total += format!(
            "{}{}", 
            numbers.first().unwrap().to_string(), 
            numbers.last().unwrap().to_string()
        ).parse::<u32>().unwrap()
    }
    println!("{}", total);  
}