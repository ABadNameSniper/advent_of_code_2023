use std::fs;
use indexmap::IndexMap;
//very happy with this one
fn main() {

    let now = std::time::Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<_> = input.trim().split(",").collect();

    let mut boxes: Vec<IndexMap<&str, u8>> = vec![IndexMap::new(); 256];
    let mut part1_sum = 0;
    for step in input {
        let op_char_index = step.rfind(is_op_char).unwrap();
        let label = &step[..op_char_index];
        let step = step.as_bytes();
        part1_sum += my_hash(step) as usize;
        let op_char = step[op_char_index];
        let selected_box = &mut boxes[my_hash(label.as_bytes()) as usize];
        match op_char {
            b'=' => selected_box.insert(label, step.last().unwrap() - 48),
            _ => selected_box.shift_remove(label)
        };
    }

    let mut sum = 0;
    for (box_index, label_box) in boxes.iter().enumerate() {
        for (order_index, key) in label_box.keys().enumerate() {
            sum += (box_index + 1) * (order_index + 1) * label_box[key] as usize;
        }
    }
    println!("{sum}, {part1_sum}, {}", now.elapsed().as_micros());
}

fn is_op_char(char: char) -> bool {
    char == '=' || char == '-'
}

fn my_hash(step: &[u8]) -> u8 {
    let mut sum = 0;
    for byte in step {
        let byte = *byte as usize;
        sum += byte;
        sum *= 17;
        sum %= 256;
    }
    sum as u8
}