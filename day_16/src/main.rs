use std::{fs, ops, collections::HashMap};
use std::cmp::max;
//I went gourmet code with this 500us solution!

fn main() {

    let now = std::time::Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<_> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    println!("Part 1: {}", energize(&grid, &mut vec![(Position { y: 0, x: 0}, EAST)]));

    let mut max_val = 0;
    for (index, line) in grid.iter().enumerate() {
        max_val = max(
            max_val,
            max (
                energize(&grid, &mut vec![(Position { y: index as isize, x: 0 + 1                       }, EAST)]),
                energize(&grid, &mut vec![(Position { y: index as isize, x: line.len() as isize - 1 - 1 }, WEST)])
            )
        )
    }
    for column in 0..grid[0].len() {
        max_val = max(
            max_val,
            max (
                energize(&grid, &mut vec![(Position { y: 0 + 1                  , x: column as isize }, SOUTH)]),
                energize(&grid, &mut vec![(Position { y: grid.len() as isize -1 , x: column as isize }, NORTH)])
            )
        )
    }

    println!("{}", max_val);
    println!("{}", now.elapsed().as_millis());

}

fn energize(grid: &Vec<Vec<u8>>, splits: &mut Vec<(Position, Position)>) -> usize {
    let mut energized_grid = vec![vec![false; grid[0].len()]; grid.len()];

    // println!("{:?}", splits.first().unwrap());

    let mut splitters: HashMap<Position, bool> = HashMap::new();

    while let Some((mut pos, mut dir)) = splits.pop() {
        while let Some(tile) = match grid[pos] {
            u8::MAX => None,
            tile => Some(tile)
        } {
            energized_grid[pos.y as usize][pos.x as usize] = true;

            match tile {
                b'.' => {  },
                b'/' => {
                    dir = match dir {
                        SOUTH => WEST,
                        WEST => SOUTH,
                        NORTH => EAST,
                        EAST => NORTH,
                        _ => panic!("Unknown direction")
                    };
                },
                b'\\' => {
                    dir = match dir {
                        SOUTH => EAST,
                        EAST => SOUTH,
                        NORTH => WEST,
                        WEST => NORTH,
                        _ => panic!("Unknown direction")
                    };
                },
                b'-' => {
                    if dir == NORTH || dir == SOUTH {
                        if splitters.contains_key(&pos) {
                            break;
                        }
                        splitters.insert(pos, true);
                        dir = EAST;
                        splits.push((pos + WEST, WEST));
                        
                    }
                },
                b'|' => {
                    if dir == EAST || dir == WEST {
                        if splitters.contains_key(&pos) {
                            break;
                        }
                        splitters.insert(pos, true);
                        dir = NORTH;
                        splits.push((pos + SOUTH, SOUTH));
                        
                    }
                },
                _ => panic!("unknown!")
            }
            pos += dir;
        }
    }

    let mut sum = 0;
    for line in &energized_grid {
        for is_energized in line {
            sum += if *is_energized { 1 } else { 0 };
        }
    }

    // println!("{}", 
    //     energized_grid
    //         .iter()
    //         .map(|line| {
    //             line
    //                 .iter()
    //                 .map(|boolean| {
    //                     if *boolean { '#' } else { '.' }
    //                 })
    //                 .collect::<String>()
                    
    //         })
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );

    // println!("{sum}");

    sum

}

const NORTH: Position = Position { y: -1, x:  0 };
const EAST: Position = Position  { y:  0, x:  1 };
const SOUTH: Position = Position { y:  1, x:  0 };
const WEST: Position = Position  { y:  0, x: -1 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    y: isize,
    x: isize
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

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl ops::Index<Position> for Vec<Vec<u8>> {
    type Output = u8;
    fn index(&self, index: Position) -> &Self::Output {
        //not the most eloquent, but I can't return an option!
        if index.y as usize >= self.len() || index.y < 0 { return &u8::MAX }
        if index.x as usize >= self[0].len() || index.x < 0 {return &u8::MAX }
        return &self[index.y as usize][index.x as usize];
        
    }
}