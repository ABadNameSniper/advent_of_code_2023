use std::fs;
use std::time::Instant;
use day_11::recurse;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

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

    const ITER: usize = 0;
    // for (records, arrangements) in lines {//[ITER..ITER+1].into_iter() {

    let mut output = String::new();

    let total: usize = lines
        // [ITER..ITER+1]
        .iter().map(|(records, arrangements)| {

        println!("{:?}, {:?}", records, arrangements);

        let depth = "";
        let last = recurse(&records, &arrangements);
        //println!("Combos: {}", last);
        let last_text = last.to_string();
        output += &format!("{last_text}\n");
        last
        
    }).sum();

    fs::write("output2.txt", output).unwrap();

    println!("{total}");
    println!("{}", now.elapsed().as_millis());
}