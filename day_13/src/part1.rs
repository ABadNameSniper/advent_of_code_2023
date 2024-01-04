use std::fs;
use std::cmp::min;
fn main() {
    let input= fs::read_to_string("input.txt").unwrap();
    let input: Vec<_>  = input
        .lines()
        .map(|line| {
            line
                .as_bytes()
                .to_vec()
        })
        .collect();

    //have to go over it twice due to issues splitting the groups...
    let input: Vec<_> = input
        .split(|line| line.is_empty())
        .collect();

    let mut horizontal_sum = 0;
    let mut vertical_sum = 0;
    for group in input {
        let reflect_result = get_line_of_reflection(group);
        if reflect_result > 0 {
            vertical_sum += reflect_result;
            continue;
        }
        let mut rotation = vec![Vec::<u8>::new(); group[0].len()];
        for line in group {
            for (byte_index, byte) in line.as_slice().iter().enumerate() {
                rotation[byte_index].push(*byte);
            }
        }
        horizontal_sum += get_line_of_reflection(&rotation);
    }
    let sum = vertical_sum * 100 + horizontal_sum;
    println!("Sum: {}", sum);
}

fn get_line_of_reflection(group: &[Vec<u8>]) -> usize {
    let mut line_of_reflection: Option<usize> = None;
    for index in 1..group.len() {
        if group[index - 1] == group[index] {
            line_of_reflection = Some(index);
            for rest in 1..min(index, group.len() - index) {
                if group[index - 1 - rest] != group[index + rest] {
                    line_of_reflection = None;
                    break;
                }
            }
            if line_of_reflection.is_some() {
                break;
            }
        }
    }
    line_of_reflection.unwrap_or(0)
}