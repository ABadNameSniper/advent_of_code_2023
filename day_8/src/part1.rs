use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let instructions: Vec<usize> = lines
    .next()
    .unwrap()
    .chars()
    .map(|char| match char { 'L' => 0, _ => 1})
    .collect();
    lines.next();
    let mut map: HashMap<&str, [&str; 2]> = HashMap::<&str, [&str; 2]>::new();
    lines.for_each(|line| {
        map.insert(&line[0..3], [&line[7..10], &line[12..15]]);
    });

    let mut key = "AAA";
    let mut direction = instructions[0];
    let mut step = 0;
    while key != "ZZZ" {
        key = map[key][direction];
        step += 1;
        direction = *instructions.get(step % instructions.len()).unwrap();
    }
    println!("{step}");
}
