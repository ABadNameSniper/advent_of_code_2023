use std::fs;
use std::time::Instant;
use day_11::recurse;
use rayon::prelude::*;
fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<(_, _)> = input
        .lines()
        .map(|line| {
            
            let line = line
                .split_whitespace()
                .collect::<Vec<_>>();
            let record = line[0]
                .as_bytes()
                .iter()
                .map(|byte| *byte)
                //add ? after ever record segment
                .chain([b'?'])
                .collect::<Vec<u8>>();
            let arrangements = line[1]
                .split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            let mut new_record = record.clone();
            // println!("{:?}", new_record);
            let mut new_arrangements = arrangements.clone();
            for _ in 0..4 {
                new_record.extend_from_slice(&record);
                new_arrangements.extend_from_slice(&arrangements);
            }
            //cut off last question
            new_record.pop();
            // println!("{:?}, \n{:?}", new_record, new_arrangements);
            (
                new_record, new_arrangements
                
            )
        })
        .collect();

    const ITER: usize = 5;
    // for (records, arrangements) in lines {//[ITER..ITER+1].into_iter() {

    let mut output = String::new();

    let total: usize = lines
        // [ITER..ITER+1]
        //this one weird trick speeds up your program 10x!
        //computer scientists hate him!
        .par_iter()
        // .iter()
        .enumerate()
        // .map(|(records, arrangements)| {
        .map(|(index, (records, arrangements))| {


        println!("{}/{}", index, lines.len());

        let depth = "";
        let last = recurse(&records, &arrangements);
        //println!("Combos: {}", last);
        // let last_text = last.to_string();
        // output += &format!("{last_text}\n");
        last
        
    }).sum();

    fs::write("output2.txt", output).unwrap();

    println!("{total}");
    println!("{}", now.elapsed().as_millis());
}