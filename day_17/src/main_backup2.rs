//This one's rough! The first or second time I've had to look for some hints on the problem itself.
//The first time I'm looking up a solution to implement into code, though.
//https://www.reddit.com/r/adventofcode/comments/18kewly/2023_day_17_never_did_pathfinding_no_clue_how_to/
//https://www.redblobgames.com/pathfinding/a-star/introduction.html
//i cheated and read the spoiled comments from u/gabrielchl and u/Few-Example3992 in that thread :flushed:

use std::{fs, collections::HashMap, ops};
use priority_queue::PriorityQueue;
use std::cmp::Reverse as rev;

const NO_DIR: Position = Position { y: 0, x: 0 };
const MAX_STRAIGHT: u8 = 30;

fn main() {

    let now = std::time::Instant::now();

    let input = fs::read_to_string("input2.txt").unwrap();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| 
            line.as_bytes().to_vec()
        )
        .collect();

    let mut pq = PriorityQueue::new();

    
    //Redblob Games (input 4)
    // let start = Position { y: 4, x: 1};
    // let goal = Position { y: 5, x: 8 };

    // //Start -> Finish
    let start = Position { y: 0, x: 0};
    let goal = Position { y: grid.len() as isize - 1, x: grid[0].len() as isize - 1};
    
    // //Finish -> Start
    // let goal = Position { y: 0, x: 0};
    // let start = Position { y: grid.len() as isize - 1, x: grid[0].len() as isize - 1};

    // pq.push(start, rev(0));
    // pq.push(start, 0);

    pq.push((start, NO_DIR), rev(0));


    let mut came_from: HashMap<Position, _> = HashMap::new();
    let mut cost_so_far: HashMap<Position, u16> = HashMap::new();
    came_from.insert(start, None::<(_, _)>);
    // came_from.insert(start, None::<(_, _, _)>);
    cost_so_far.insert(start, 0);

    let mut display_grid = grid.clone();

    while let Some((current, _)) = pq.pop() {
        let current_dir = current.1;
        let current = current.0;
        if goal == current {
            println!("GOAL");
            break;
        }

        let rev_dir = Position { y: -current_dir.y, x: -current_dir.x };

        for dir in DIRECTIONS {

            let next = current + dir;
            //backwards or OOB
            if dir == rev_dir || grid[next] == u8::MAX { 
                continue 
            }

            let mut consecutive = 0;
            let mut prev = came_from[&current];
            for _ in 0..MAX_STRAIGHT {
                let exists = prev.is_some() && came_from.contains_key(&prev.unwrap().0);
                if exists {
                    if dir != prev.unwrap().1 {
                        break
                    } else {
                        consecutive += 1;
                    }
                    prev = came_from[&prev.unwrap().0];
                } else {
                    break
                }
            }
            if consecutive >= MAX_STRAIGHT { continue }

            let heat_loss = grid[next] as u16 - 48;
            let current_cost = cost_so_far[&current];
            let new_cost = current_cost + heat_loss;

            //if undescovered                   or you can get there cheaper
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                // display_grid[next] = b'#';
                display_grid[next] = show_dir(dir);
                // display_grid[next] = new_cost as u8 + 48;
                cost_so_far.insert(next, new_cost);
                // let priority = new_cost;

                let priority = rev(new_cost);
                // pq.push(next, rev(priority));
                // pq.push(next, priority);
                pq.push((next, dir), priority);
                came_from.insert(next, Some((current, dir)));
            }
        }
    }

    println!("{}", format_grid(display_grid) );

    let mut path_traveled = grid.clone();
    let mut heat_loss = 0;
    let mut last = goal;
    println!("{}", last);
    let mut dir = came_from[&last].unwrap().1;

    while let Some(pair) = came_from[&last] {
        path_traveled[last] = show_dir(dir);
        heat_loss += grid[last] as usize - 48;
        (last, dir) = pair;
    }

    println!("{}", format_grid(path_traveled));
    println!("{}", heat_loss);
    print!("{}", now.elapsed().as_millis());

}

fn format_grid(grid: Vec<Vec<u8>>) -> String {
    grid
        .iter()
        .map(|line| {
            std::str::from_utf8(line).unwrap().to_owned() + "\n"
        })
        .collect::<String>()
}

const NORTH: Position = Position { y: -1, x:  0 };
const EAST: Position = Position  { y:  0, x:  1 };
const SOUTH: Position = Position { y:  1, x:  0 };
const WEST: Position = Position  { y:  0, x: -1 };

const DIRECTIONS: [Position; 4] = [NORTH, EAST, SOUTH, WEST];

fn show_dir(dir: Position) -> u8 {
    match dir {
        NORTH => b'^',
        EAST => b'>',
        SOUTH => b'v',
        WEST => b'<',
        _ => b'*'
    }
}

//Copied from a previous day. I think next advent of code I'll just make a single package with a bunch of either workspaces or binaries.
//That way I can put more stuff into libraries.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    y: isize,
    x: isize
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position Y:{}, X:{}", self.y, self.x)
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

impl ops::Index<Position> for Vec<Vec<u8>> {
    type Output = u8;
    fn index(&self, index: Position) -> &Self::Output {
        //not the most eloquent, but I can't return an option!
        if index.y as usize >= self.len() || index.y < 0 { return &u8::MAX }
        if index.x as usize >= self[0].len() || index.x < 0 {return &u8::MAX }
        return &self[index.y as usize][index.x as usize];
        
    }
}

impl ops::IndexMut<Position> for Vec<Vec<u8>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        if index.x < 0 || index.y < 0 || index.y as usize > self.len() || index.x as usize > self[0].len() {
            panic!("Position Index is not in bounds! Index y:{}, x:{} vs Grid Size y:{} x:{}", index.y, index.x, self.len(), self[0].len());
        } 
        &mut self[index.y as usize][index.x as usize]
    }
}