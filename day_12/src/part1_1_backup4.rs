use std::cmp::min;
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();


    let lines: Vec<(_, _)> = input
        .lines()
        .map(|line| {
            
            let line = line
                .split_whitespace()
                .collect::<Vec<_>>();
            (
                line[0]
                    .as_bytes()
                    .iter()
                    .map(|byte| *byte)
                    .collect::<Vec<u8>>(), 
                line[1]
                    .split(",")
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            )
        })
        .collect();

    const ITER: usize = 60;
    // for (records, arrangements) in lines {//[ITER..ITER+1].into_iter() {

    let mut output = String::new();

    let total: usize = lines
        // [ITER..ITER+1]
        .iter().map(|(records, arrangements)| {

    
        //println!("{}, {}, {}", records.len(), arrangements.len(), arrangements.iter().sum::<u8>());
        let slack: usize = 
            records.len()
            + 1
            - arrangements.len() 
            - arrangements.iter().sum::<u8>() as usize;
            

        //println!("Initial slack: {}", slack);

        let depth = "";
        let last = recurse(&records, &arrangements, slack, depth);
        //println!("Combos: {}", last);
        let last_text = last.to_string();
        output += &format!("{last_text}\n");
        last
        
    }).sum();

    fs::write("output2.txt", output).unwrap();

    println!("{total}");
    println!("{}", now.elapsed().as_millis());
}

fn recurse(records: &[u8], arrangements: &[u8], slack: usize, depth: &str) -> usize {
    if arrangements.len() == 0 {
        if records.iter().any(|byte| *byte == b'#') {
            //println!("{depth}Forgot something!");
            return 0;
        }
        //println!("{depth}Nothing left here! Looks good");
        return 1;
    }
    //println!("{depth}Recurse with slack: {}, records: {:?}, arrangements: {:?}", slack, std::str::from_utf8(records).unwrap(), arrangements);

    let group_size = *arrangements.first().unwrap() as usize;
    let mut offset = 0;
    let mut total: usize = 0;
    // for offset in 0..slack {
    while offset <= slack {
        let records = &records[offset..];

        //println!("{depth}Offset: {}, Slack: {}, Records: {}", offset, slack, std::str::from_utf8(records).unwrap());

        // if records.is_empty() {
        //     //println!("{depth}Odd. Records are empty");
        //     return total;
        // }
        // if group_size > records.len() {
        //     //println!("{depth}no room!");
        //     return total;
        // }

        if //check if the one right after group would mesh together
            (group_size) < records.len() &&
            records[
                group_size
            ] == b'#' 
        {
            //println!("{depth}smooshed into {}", records[group_size] as char);
            if *records.first().unwrap() == b'#' {
                //println!("{depth}smooshed but have to place here! invalid");
                break;
            }
            offset += 1;
            continue;
        }

        let breathing_room = match records[group_size - 1] == b'#' {
            true => 1,
            false => 0
        };

        let potential_group = &records[0..min(group_size, records.len())];
        //if no dots, it's a match
        if !potential_group.iter().any(|byte| *byte == b'.') {
            //println!("{depth}the shoe {} fits in {:?}", group_size, &potential_group);
            total += recurse(
                &records[min(1 + group_size /*+ breathing_room*/, records.len())..],
                &arrangements[1..], 
                slack - offset, //decrease by offset? offset and group_size?
                format!(" {depth}").as_str()
            );
            if 
                //not sure on qualifier
                *records.first().unwrap() == b'#' 
                // records.iter().any(|byte| *byte == b'#')
            {
                //println!("{depth}char found in area. must place here?");
                //only valid offset?
                break;
            }
        } else {
            if *records.first().unwrap() == b'#' {
                //println!("{depth}can't skip this!");
                break;
            }
        }
        offset += 1
    }
    
    //println!("{depth}Went through all \"{}\" offsets", slack + 1);

    total

}

//    ???????#?????#..?? 5,2

// 72xx -> 7133

//    ?#..#?.?#?#?????? 1,1,2,5