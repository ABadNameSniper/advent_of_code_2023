use std::fs;
const CYCLES: usize = 1_000_000_000;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    //it's much easier to do it on its side.
    //rotate once to start on the north
    //cycles END in the East, so rotate it twice
    //to let the loop rotate it back once
    let mut rotation = rotate(input);
    rot_out(&rotation, "initial_rot.txt");
    rotation = rotate(rotation);

    let mut prev_rots: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut cycle_results = Vec::new();
    let mut found_previous = false;

    for cycle in 0..CYCLES {
        for _ in 0..4 {
            rotation = rotate(rotation);

            rotation.iter_mut().for_each(|line| {
                let mut last_open = 0;
                for current_index in 0..line.len() {
                    let current_byte = line.get_mut(current_index).unwrap();
                    let byte_val = *current_byte;

                    match byte_val {
                        b'#' => last_open = current_index + 1,
                        b'O' => {
                            *current_byte = b'.';
                            line[last_open] = byte_val;
                            last_open += 1;
                        },
                        _ => {}
                    }
                }
            });
        }
        
        for 
            (index, prev_rot) 
            in prev_rots.clone().iter().enumerate().rev() 
        {
            if *prev_rot == rotation {    
                found_previous = true;
                println!("same! at {cycle}, {index}");
                rot_out(&rotation, "rotation.txt");
                rot_out(&prev_rot, "prevrot.txt");
                println!("results: {}", cycle_results[cycle - 9]);
                cycle_results = cycle_results[index..cycle].to_vec();
                println!("{}", cycle_results[(CYCLES - index - 1) % cycle_results.len()]);
                break;
            }
        }
        if found_previous { break }


        let mut adjusted_rot = rotate(rotation.clone());
        adjusted_rot = rotate(adjusted_rot);
        cycle_results.push(calc_load(&adjusted_rot));
        prev_rots.push(rotation.clone());
    }

    println!("{:?}", cycle_results);

    rot_out(&rotation, "output.txt");
}

//Clockwise rotation i think
fn rotate(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotation: Vec<Vec<u8>> = vec![vec![0; grid.len()]; grid[0].len()];
    for (line_index, line) in grid.iter().enumerate() {
        for (byte_index, byte) in line.as_slice().iter().enumerate() {
            rotation[byte_index][line.len() - 1 - line_index] = *byte;
        }
    }
    rotation
}

fn calc_load(grid: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for (index, line) in grid.iter().enumerate() {
        for byte in line {
            if *byte == b'O' {
                sum +=  line.len() - index
            }
        }
    }
    return sum;
}

fn rot_out(rotation: &Vec<Vec<u8>>, name: &str) {
    fs::write(
        name, 
        rotation
            .iter()
            .map(|line| std::str::from_utf8(&line).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    ).unwrap();
}