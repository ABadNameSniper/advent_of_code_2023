use std::fs;
use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"#+").unwrap();

    let lines: Vec<(_, _)> = input
        .lines()
        .map(|line| {
            
            let line = line
                .split_whitespace()
                .collect::<Vec<_>>();
            (
                line[0]
                    .as_bytes()
                    .iter()
                    .map(|byte| *byte)
                    .collect::<Vec<u8>>(), 
                line[1]
                    .split(",")
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            )
        })
        .collect();

    let mut output = String::new();
    let mut sum = 0;
    for (records, arrangements) in lines {
        //i cannot be bothered to think of a clever, recursive way to solve this problem
        //so i will brute force it.

        //gee whizz I love my dataset being so small that i can just iterate
        //over every possible combination and check if each is correct!

        //get locations of unknown indicies
        let mut unknowns = Vec::new();
        records.iter().enumerate().for_each(|(index, byte)|
            if *byte == b'?' {
                unknowns.push(index);
            }
        );

        let mut attempt_sum = 0;

        for attempt in 0..(0b0000_0001 << unknowns.len()) {
            let mut records = records.clone();
            for (nth_index, records_index) in unknowns.iter().enumerate() {
                records[*records_index] = match attempt & (1 << nth_index) > 0 {
                    true => b'#',
                    false => b'.'
                }
            }

            // println!("{:?}", std::str::from_utf8(&records).unwrap());

            let captures: Vec<_> = re.captures_iter(std::str::from_utf8(&records).unwrap()) 
            .map(|c| c.extract()).collect();

            if captures.len() != arrangements.len() {
                continue;
            }

            let mut fail = 1;
            
            //Parse to check if it's correct.
            for (index, (captured, [])) in captures.iter().enumerate() {
                //mismatch = fail
                if arrangements[index] != captured.len() as u8 { 
                    fail = 0;
                    break 
                }
            }
            // output = format!("{output}\n{}", fail.to_string());
            
            attempt_sum += fail;
        }
        //println!("{}", now.elapsed().as_millis());
        output = output + &attempt_sum.to_string() + "\n";

        sum += attempt_sum;
        
    }
    fs::write("output1.txt", output).unwrap();
    println!("{}", now.elapsed().as_millis());
    println!("{sum}");
}