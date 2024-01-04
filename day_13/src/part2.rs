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
        let (equals, mut smudged) = smudge_equal(&group[index - 1], &group[index], false);
        if equals {
            line_of_reflection = Some(index);
            for rest in 1..min(index, group.len() - index) {
                let (still_equals, currently_smudged) = smudge_equal(&group[index - 1 - rest], &group[index + rest], smudged);
                if !still_equals {
                    line_of_reflection = None;
                    break;
                }
                smudged = currently_smudged;
            }
            //MUST be the smudged mirror
            if smudged && line_of_reflection.is_some() {
                break;
            }
            line_of_reflection = None;
        }
    }
    line_of_reflection.unwrap_or(0)
}

fn smudge_equal(a: &Vec<u8>, b: &Vec<u8>, pre_smudged: bool) -> (bool, bool) {
    let mut smudged = pre_smudged;
    for byte_index in 0..a.len() {
        if a[byte_index] != b[byte_index] {
            if smudged {
                //too messy
                return (false, smudged);
            }
            smudged = true;
        }
    }
    return (true, smudged);
}