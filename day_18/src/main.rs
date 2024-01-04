use std::{fs, collections::{VecDeque, HashMap}, ops};

//hmmm... find tiles within a loop you say?
//Unfortunately my code is structured poorly (hey it's my first year doing AOC, in a language I'm just learning!)
//So I'll just be copying and pasting parts from day 10 here.


fn main() {


    let tiles: HashMap<u8, (Position, Position)> = HashMap::from([
        (b'L', (NORTH, EAST)),
        (b'J', (NORTH, WEST)),
        (b'|', (NORTH, SOUTH)),
        (b'7', (SOUTH, WEST)),
        (b'F', (SOUTH, EAST)),
        (b'-', (EAST, WEST)),
    ]);
    ////inverse;
    // let tiles: HashMap<u8, (Position, Position)> = HashMap::from([
    //     (b'L', (SOUTH, WEST)),
    //     (b'J', (SOUTH, EAST)),
    //     (b'|', (SOUTH, NORTH)),
    //     (b'7', (NORTH, EAST)),
    //     (b'F', (NORTH, WEST)),
    //     (b'-', (WEST, EAST)),
    // ]);


    

    let input = fs::read_to_string("input.txt").unwrap();

    let mut dirs: Vec<Position> = Vec::new();
    let mut amounts: Vec<u8> = Vec::new();
    let mut edge_colors: Vec<u32> = Vec::new();


    let mut last_dir = NO_DIR;

    input
        .lines()
        .for_each(|line| {
            let mut chunks = line.split_ascii_whitespace();
            dirs.push(
                match *chunks
                    .next()
                    .unwrap()
                    .as_bytes()
                    .first()
                    .unwrap() 
                {
                    b'U' => NORTH,
                    b'R' => EAST,
                    b'L' => WEST,
                    b'D' => SOUTH,
                    _ => NO_DIR
                }
            );
            amounts.push(
                chunks
                .next()
                .unwrap()
                .parse::<u8>()
                .unwrap()
            );
            
            edge_colors.push(u32::from_str_radix((chunks
                    .next()
                    .unwrap()
                    .split_at(2)
                )
                .1
                .split_at(6).0,
            16
            ).unwrap());
        });

        //will probably overflow                        assume it always gets back to start
        // let most_case_scenario = amounts.iter().sum::<u8>() as usize / 2;
        // let most_case_scenario = 83284;
        let most_case_scenario = amounts.iter().map(|val| *val as usize).sum();

        let mut row: VecDeque<u8> = VecDeque::with_capacity(most_case_scenario);
        let mut pipes: VecDeque<VecDeque<_>> = VecDeque::with_capacity(most_case_scenario);
        let mut color_row: VecDeque<u32> = VecDeque::with_capacity(most_case_scenario);
        let mut colors: VecDeque<VecDeque<_>> = VecDeque::with_capacity(most_case_scenario);

        let mut target = Position { y: 0, x: 0};

        let mut perimeter = 0;

        for (index, dir) in dirs.iter().enumerate() {
            let pipe = get_pipe(last_dir, dir, &tiles);
            

            let horizontal = match *dir {
                NORTH => false,
                SOUTH => false,
                _ => true
            };

            //probably unnecessary. Start at 1?
            let mut first = true;

            //all checks
            if target.y < 0 {
                pipes.push_front(row.clone());
                colors.push_front(color_row.clone());
            } else if target.y == pipes.len() as isize {
                pipes.push_back(row.clone());
                colors.push_back(color_row.clone());
            }
            if target.x < 0 {
                for (index, pipe_row) in pipes.iter_mut().enumerate() {
                    pipe_row.push_front(b'.');
                    colors[index].push_front(b'.' as u32);
                    row.push_front(b'.');
                    color_row.push_front(b'.' as u32);
                }
            } else if target.x == pipes.front().unwrap().len() as isize {
                for (index, pipe_row) in pipes.iter_mut().enumerate() {
                    pipe_row.push_back(0);
                    colors[index].push_back(b'.' as u32);
                    row.push_back(b'.');
                    color_row.push_back(b'.' as u32);
                }
            }

            println!("{}", target);
            if horizontal {
                for _ in 0..amounts[index] {
                    perimeter += 1;
                    pipes[target] = if first { pipe } else { b'-' };
                    colors[target] = edge_colors[index];
                    first = false;
                    target = target + *dir;
                    if target.x < 0 {
                        for (index, pipe_row) in pipes.iter_mut().enumerate() {
                            pipe_row.push_front(b'.');
                            colors[index].push_front(b'.' as u32);
                            row.push_front(b'.');
                            color_row.push_front(b'.' as u32);
                        }
                    } else if target.x == pipes.front().unwrap().len() as isize {
                        for (index, pipe_row) in pipes.iter_mut().enumerate() {
                            pipe_row.push_back(0);
                            colors[index].push_back(b'.' as u32);
                            row.push_back(b'.');
                            color_row.push_back(b'.' as u32);
                        }
                    }
                }
            } else {
                for _ in 0..amounts[index] {
                    perimeter += 1;
                    pipes[target] = if first { pipe } else { b'|' };
                    colors[target] = edge_colors[index];
                    first = false;
                    target = target + *dir;
                    if target.y < 0 {
                        pipes.push_front(row.clone());
                        colors.push_front(color_row.clone());
                    } else if target.y == pipes.len() as isize {
                        pipes.push_back(row.clone());
                        colors.push_back(color_row.clone());
                    }
                }
            }

            // last_dir = *dir;
            last_dir = Position { y: -dir.y, x: -dir.x };

        }

    

    // let first = dirs.first().unwrap();
    // println!("pipe: {}", get_pipe(last_dir, &first, &tiles) as char);
    // pipes[target] = get_pipe(last_dir, &first, &tiles);

    println!("{}", perimeter);

    pipes[target] = b'S';

    let finished = pipes
        .iter()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| 
            std::str::from_utf8(
                line
                    .iter()
                    .map(|byte_ref| *byte_ref)
                    .collect::<Vec<_>>()
                    .as_slice()
            )
                .unwrap()
                .to_owned()
                + "\n"
        )
        .collect::<String>();

    println!("{}", finished);

    fs::write("day10_input.txt", finished).unwrap();
}



const NO_DIR: Position = Position { y: 0, x: 0 };

const NORTH: Position = Position { y: -1, x:  0 };
const EAST: Position = Position  { y:  0, x:  1 };
const SOUTH: Position = Position { y:  1, x:  0 };
const WEST: Position = Position  { y:  0, x: -1 };

fn get_pipe(last_dir: Position, dir: &Position, tiles: &HashMap<u8, (Position, Position)>) -> u8 {
    for (tile, (dir1, dir2)) in tiles.iter() {
        if (last_dir == *dir1 && *dir == *dir2) || (last_dir == *dir2 && *dir == *dir1) {
            return *tile;
        }
    }
    return b'*';
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    y: isize,
    x: isize
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NORTH => write!(f, "NORTH"),
            SOUTH => write!(f, "SOUTH"),
            EAST => write!(f, "EAST"),
            WEST => write!(f, "WEST"),
            _ => write!(f, "Position Y:{}, X:{}", self.y, self.x),
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        return Position { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

impl ops::Index<Position> for VecDeque<VecDeque<u8>>{
    type Output = u8;
    fn index(&self, index: Position) -> &Self::Output {
        //not the most eloquent, but I can't return an option!
        if index.y as usize >= self.len() || index.y < 0 { return &u8::MAX }
        if index.x as usize >= self[0].len() || index.x < 0 {return &u8::MAX }
        return &self[index.y as usize][index.x as usize];
    }
}

impl ops::IndexMut<Position> for VecDeque<VecDeque<u8>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        if index.x < 0 || index.y < 0 || index.y as usize > self.len() || index.x as usize > self[0].len() {
            panic!("Position Index is not in bounds! Index y:{}, x:{} vs Grid Size y:{} x:{}", index.y, index.x, self.len(), self[0].len());
        } 
        &mut self[index.y as usize][index.x as usize]
    }
}



impl ops::Index<Position> for VecDeque<VecDeque<u32>>{
    type Output = u32;
    fn index(&self, index: Position) -> &Self::Output {
        //not the most eloquent, but I can't return an option!
        if index.y as usize >= self.len() || index.y < 0 { return &u32::MAX }
        if index.x as usize >= self[0].len() || index.x < 0 {return &u32::MAX }
        return &self[index.y as usize][index.x as usize];
    }
}

impl ops::IndexMut<Position> for VecDeque<VecDeque<u32>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        if index.x < 0 || index.y < 0 || index.y as usize > self.len() || index.x as usize > self[0].len() {
            panic!("Position Index is not in bounds! Index y:{}, x:{} vs Grid Size y:{} x:{}", index.y, index.x, self.len(), self[0].len());
        } 
        &mut self[index.y as usize][index.x as usize]
    }
}