use std::{fs, collections::HashMap};

fn main() {
    let now = std::time::Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut position = [0, 0];

    for (line_num, line) in lines.iter().enumerate() {
        let found = line.iter().position(|byte| *byte == b'S');
        if found.is_some() {
            position = [line_num as i16, found.unwrap() as i16];
            break;
        }
    } 

    println!("Animal Position: {:?}", position);

    const NORTH: [i16; 2] = [-1,  0];
    const SOUTH: [i16; 2] = [ 1,  0];
    const EAST : [i16; 2] = [ 0,  1];
    const WEST : [i16; 2] = [ 0, -1];
    const DIRECTIONS: [[i16; 2]; 4] = [NORTH, EAST, SOUTH, WEST];

    let tiles: HashMap<u8, [[i16; 2]; 2]> = HashMap::from([
        (b'|', [NORTH, SOUTH]),
        (b'-', [EAST, WEST]),
        (b'L', [NORTH, EAST]),
        (b'J', [NORTH, WEST]),
        (b'7', [WEST, SOUTH]),
        (b'F', [SOUTH, EAST])
    ]);

    // let mut tile = &'S';
    
    let mut steps: u32 = 1;
    for direction in DIRECTIONS {
        //Find initial direction to travel in
        let mut reverse = reverse_dir(direction);

        let mut prospective_position = 
            [
                position[0] + direction[0],
                position[1] + direction[1]
            ];

        let mut prospective_tile = lines[prospective_position[0] as usize][prospective_position[1] as usize];

        if prospective_tile == b'.' { continue }

        let ends = tiles[&prospective_tile];
        if ends.iter().any(|dir| *dir == reverse) {
            //keep traveling in the direction the pipes say to go that you didn't already travel down
            while prospective_tile != b'S' {
                let tile = prospective_tile;
                position = prospective_position;
                let new_dir = **tiles[&tile]
                    .iter()
                    .filter(|dir| **dir != reverse)
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap();

                prospective_position = [position[0] + new_dir[0], position[1] + new_dir[1]];

                prospective_tile = lines[
                        prospective_position[0] as usize
                    ][
                        prospective_position[1] as usize
                ];

                reverse = reverse_dir(new_dir);

                steps += 1;
            }

            //Starting direction already found, no need go through another
            break;
        }
    }

    //halfway there, halfway back
    println!("{}", steps/2);
    println!("{}", now.elapsed().as_micros());
}
fn reverse_dir(dir: [i16; 2]) -> [i16; 2] {
    [0 - dir[0], 0 - dir[1]]
}