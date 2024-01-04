use std::{fs, collections::HashMap};

const NORTH: [i16; 2] = [-1,  0];
const SOUTH: [i16; 2] = [ 1,  0];
const EAST : [i16; 2] = [ 0,  1];
const WEST : [i16; 2] = [ 0, -1];
const DIRECTIONS: [[i16; 2]; 4] = [NORTH, EAST, SOUTH, WEST];

//Once I figured out a method, this one was really fun!
//I feel like I went a little overboard with typecasting,
//but i'm happy with what I came up with

fn main() {
    let now = std::time::Instant::now();

    let input = fs::read_to_string("day10_input.txt").unwrap();

    let lines: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut position = [0, 0];

    for (line_num, line) in lines.iter().enumerate() {
        let found = line.iter().position(|byte| *byte == b'S');
        if found.is_some() {
            position = [line_num as i16, found.unwrap() as i16];
            break;
        }
    } 


    //Make a larger copy of the grid to expand the distance between the pipes
    let mut wumbo = vec![vec![b'.'; lines[0].len()*3]; lines.len()*3];

    set_wumbo(&mut wumbo, position, [0, 0], b'S');

    println!("Animal Position: {:?}", position);

    let tiles: HashMap<u8, [[i16; 2]; 2]> = HashMap::from([
        (b'|', [NORTH, SOUTH]),
        (b'-', [EAST, WEST]),
        (b'L', [NORTH, EAST]),
        (b'J', [NORTH, WEST]),
        (b'7', [WEST, SOUTH]),
        (b'F', [SOUTH, EAST])
    ]);


    let mut steps: u32 = 1;
    for direction in DIRECTIONS {
        //Find initial direction to travel in
        let mut reverse = reverse_dir(direction);

        let mut prospective_position = 
            [
                position[0] + direction[0],
                position[1] + direction[1]
            ];

        if !inbounds(prospective_position, &lines) { continue }

        let mut prospective_tile = lines[prospective_position[0] as usize][prospective_position[1] as usize];

        if prospective_tile == b'.' { continue }

        //check if an end of the pipe is facing the animal
        let ends = tiles[&prospective_tile];
        if ends.iter().any(|dir| *dir == reverse) {
            //put a blocker next to the animal
            set_wumbo(&mut wumbo, position, direction, b'+');

            //keep travelling down the other end of the pipes until the animal is found again
            while prospective_tile != b'S' {
                let tile = prospective_tile;
                position = prospective_position;
                let new_dir = **tiles[&tile]
                    .iter()
                    .filter(|dir| **dir != reverse)
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap();

                set_wumbo(&mut wumbo, prospective_position, [0, 0], prospective_tile);

                tiles[&tile].iter().for_each(|dir| {
                    set_wumbo(&mut wumbo, prospective_position, *dir, b'+');
                });

                prospective_position = [position[0] + new_dir[0], position[1] + new_dir[1]];

                prospective_tile = lines[prospective_position[0] as usize][prospective_position[1] as usize];

                reverse = reverse_dir(new_dir);

                steps += 1;

            }

            //put a blocker next to the animal
            set_wumbo(&mut wumbo, prospective_position, reverse, b'+');

            //Starting direction already found, no need go through another
            break;
        }
    }

    //Part 1 solution
    //halfway there, halfway back
    println!("{}", steps/2);
    println!("{}", now.elapsed().as_micros());

    let mut battleship: HashMap<(u16, u16), _> = HashMap::new();
    //seems to consistantly be empty across given examples and my input
    battleship.insert((0, 0), false);
    
    let finished = wumbo.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n");
    fs::write("output.txt", finished).unwrap();

    while battleship.len() > 0 {
        for ((y, x), old) in battleship.clone().iter() {
            if !old {
                for direction in DIRECTIONS {
                    let prospective_position = [*y as i16 + &direction[0], *x as i16 + &direction[1]];
                    if !inbounds(prospective_position, &wumbo) ||
                        wumbo
                            [prospective_position[0] as usize]
                            [prospective_position[1] as usize] 
                        != b'.' 
                    { 
                        continue 
                    }

                    let prospective_positions = &(prospective_position[0] as u16, prospective_position[1] as u16);
                    if battleship.get(prospective_positions).is_none() {
                        battleship.insert(*prospective_positions, false);
                    }
                }
                
                *battleship.get_mut(&(*y, *x)).unwrap() = true;
            } else {
                wumbo[*y as usize][*x as usize] = b'X';
                battleship.remove(&(*y, *x));
            }
        }
    }

    let finished = wumbo.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n");
    fs::write("output2.txt", finished).unwrap();

    let mini = wumbo
        .iter()
        .skip(1)
        .step_by(3)
        .map(|line| {
            line
                .iter()
                .skip(1)
                .step_by(3)
                .map(|byte| *byte)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    fs::write("output3.txt", mini.iter().map(|line| std::str::from_utf8(line).unwrap()).collect::<Vec<_>>().join("\n")).unwrap();

    let mut outside = 0;
    mini.iter().for_each(|line|
        line.iter().for_each(|byte| {
            if *byte == b'X' {
                outside += 1;
            }
        })
    );

    println!("Width: {}, Height: {}, Total: {}, Pipes: {}, Outside: {}", lines[0].len(), lines.len(), lines[0].len() * lines.len(), steps, outside);

    println!("Inside tiles: {}", (lines[0].len() * lines.len()) as u32 - steps as u32 - outside);

    println!("{}", now.elapsed().as_millis());
}
fn reverse_dir(dir: [i16; 2]) -> [i16; 2] {
    [0 - dir[0], 0 - dir[1]]
}

fn inbounds(guess: [i16; 2], grid: &Vec<Vec<u8>>) -> bool {
    guess[0] >= 0 && (guess[0] as usize) < grid.len() &&
    guess[1] >= 0 && (guess[1] as usize) < grid[0].len()
}

fn set_wumbo(wumbo: &mut Vec<Vec<u8>>, position: [i16; 2], direction: [i16; 2], byte: u8) {
    wumbo
        [(position[0] * 3 + direction[0] + 1) as usize]
        [(position[1] * 3 + direction[1] + 1) as usize]
        = byte;
}