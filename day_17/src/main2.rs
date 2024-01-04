//get me outta here! i want to be done with pathfinding!!
use std::{fs, collections::HashMap, ops};
use priority_queue::PriorityQueue;
use std::cmp::Reverse as rev;

const NO_DIR: Position = Position { y: 0, x: 0 };
const MAX_STRAIGHT: u8 = 10;

fn main() {

    let now = std::time::Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| 
            line.as_bytes().to_vec()
        )
        .collect();

    let mut pq = PriorityQueue::new();

    // //Start -> Finish
    let start = Position { y: 0, x: 0};
    let goal = Position { y: grid.len() as isize - 1, x: grid[0].len() as isize - 1};


    let start_key = (start, NO_DIR, 0);

    pq.push(start_key, rev(0));


    let mut came_from: HashMap<_, _> = HashMap::new();
    let mut cost_so_far: HashMap<_, u16> = HashMap::new();
    came_from.insert(start_key, None::<(_, _, _)>);
    // came_from.insert(start, None::<(_, _, _)>);
    cost_so_far.insert(start_key, 0);

    let mut display_grid = grid.clone();

    let mut last = start_key;//default val

    while let Some((current_key, _)) = pq.pop() {
        let prev_dir = current_key.1;
        let current = current_key.0;
        let chain = current_key.2;
        if goal == current {
            println!("cost {}", cost_so_far[&current_key]);
            println!("GOAL, chain: {}", chain);
            if chain < 4 || chain > MAX_STRAIGHT {
                continue
            }
            last = current_key;
            break;
        }

        let rev_dir = Position { y: -prev_dir.y, x: -prev_dir.x };

        for dir in DIRECTIONS {

            let next = current + dir;
            //backwards or OOB
            if dir == rev_dir || grid[next] == u8::MAX { 
                continue 
            }

            let mut consecutive: u8 = chain;
            
            if consecutive > 0 && consecutive < 4 && dir != prev_dir {
                continue;
            }
            if prev_dir == dir {
                consecutive += 1;
            } else {
                consecutive = 1;
            }
            if consecutive > MAX_STRAIGHT {
                continue;
            }

            let heat_loss = grid[next] as u16 - 48;
            let current_cost = cost_so_far[&current_key];
            let new_cost = current_cost + heat_loss;

            let next_key = (next, dir, consecutive);

            //if undescovered                   or you can get there cheaper
            if !cost_so_far.contains_key(&next_key) || new_cost < cost_so_far[&next_key] {
                // display_grid[next] = b'#';
                display_grid[next] = show_dir(dir);
                // display_grid[next] = new_cost as u8 + 48;
                cost_so_far.insert(next_key, new_cost);

                let priority = rev(new_cost);
                pq.push(next_key, priority);
                came_from.insert(next_key, Some(current_key));
            }
        }
    }

    println!("{}", format_grid(display_grid) );

    let mut path_traveled = grid.clone();
    let mut heat_loss = 0;
    println!("{:?}", last);
    
    loop {
        let (pos, dir, chain) = last;
        path_traveled[pos] = show_dir(dir);
        heat_loss += grid[pos] as u16 - 48;

        if !came_from.contains_key(&last) {
            break;
        }
        let last_opt = came_from[&last];
        if last_opt.is_none() {
            break;
        } 
        last = last_opt.unwrap();
    }
    heat_loss -= grid[start] as u16 - 48;
    println!("hi newline");
    println!("{}", format_grid(path_traveled));
    println!("hl: {heat_loss}");

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