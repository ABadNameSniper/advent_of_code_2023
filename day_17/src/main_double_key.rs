//This one's rough! The first or second time I've had to look for some hints on the problem itself.
//The first time I'm looking up a solution to implement into code, though.
//https://www.reddit.com/r/adventofcode/comments/18kewly/2023_day_17_never_did_pathfinding_no_clue_how_to/
//https://www.redblobgames.com/pathfinding/a-star/introduction.html
//i cheated and read the spoiled comments from u/gabrielchl and u/Few-Example3992 in that thread :flushed:

use std::{fs, collections::HashMap, ops};
use priority_queue::PriorityQueue;
use std::cmp::Reverse as rev;

const NO_DIR: Position = Position { y: 0, x: 0 };

fn main() {

    let input = fs::read_to_string("input2.txt").unwrap();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| 
            line.as_bytes().to_vec()
        )
        .collect();

    let mut pq: PriorityQueue<((Position, Position), u16), rev<u16>> = PriorityQueue::new();


    let start = (Position { y: 0, x: 0}, NO_DIR);
    let goal = Position { y: grid.len() as isize - 1, x: grid[0].len() as isize - 1};

    pq.push((start, 0), rev(0));


    let mut came_from: HashMap<(Position, Position), _> = HashMap::new();
    let mut cost_so_far: HashMap<(Position, Position), u16> = HashMap::new();
    // came_from.insert(start, None);
    // cost_so_far.insert(start, 0);

    let mut display_grid = grid.clone();

    let mut last = start;//placeholder

    while let Some(((current, current_cost), _)) = pq.pop() {

        if came_from.contains_key(&current) {
            println!("If I pull out an element that's already in the dictionary, I can just discard instead");
            continue;
        }

        //"I got around this case by adding the (location,direction) to the dictionary when I pull it out as the lowest score from the heap, 
        //not when I add to the heap. That way whichever way is quicker will be pulled out first and be the one I use in the path."
        let current_dir = current.1;
        let current_pos = current.0;

        let rev_dir = Position { y: -current_dir.y, x: -current_dir.x }; 
        came_from.insert(current, (current_pos + rev_dir, current_dir));
        cost_so_far.insert(current, current_cost);
        if goal == current_pos {
            println!("GOAL");
            last = current;
            break;
        }

        for dir in DIRECTIONS {
            let next_pos = current_pos + dir;
            let next = (next_pos, dir);
            
            if rev_dir == dir { continue }

            if grid[next_pos] == u8::MAX { continue }

            let heat_loss = grid[next_pos] as u16 - 48;
            let current_cost = cost_so_far[&current];
            let new_cost = current_cost + heat_loss;

            display_grid[next_pos] = show_dir(dir);
            // display_grid[next_pos] = new_cost as u8 + 48;

            println!("pushed {}", new_cost);
            pq.push(((next), new_cost), rev(new_cost));
        }
    }

    println!("{}", format_grid(display_grid) );

    let mut path_traveled = grid.clone();
    let mut heat_loss = 0;

    println!("Heat loss 1: {}", cost_so_far[&last] + grid[last.0] as u16 - 48);
    loop {
        println!("Last: {:?}", last);
        path_traveled[last.0] = show_dir(last.1);

        if !came_from.contains_key(&last) {
            break;
        }
        heat_loss += grid[last.0];
        last = came_from[&last];
        
    }

    println!("{:?}", last);
    


    println!("{}", format_grid(path_traveled));
    println!("HL 2: {}", heat_loss);

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