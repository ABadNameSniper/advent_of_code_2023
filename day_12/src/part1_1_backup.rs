use std::cmp::min;
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input3.txt").unwrap();


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

    let mut sum = 0;
    const ITER: usize = 0;
    for (records, arrangements) in lines[ITER..ITER+1].into_iter() {
        println!("{}, {}, {}", records.len(), arrangements.len(), arrangements.iter().sum::<u8>());
        let slack: usize = 
            records.len()
            + 1 //extra item since length of arrangements is more than num of groups
            //possibly subtract amount of known broken pipe?
            //i don't think it helps find a solution, but may help on time
            //- records.iter().map(|byte| match *byte == b'#' { true => 1, false => 0}).sum::<usize>()
            - arrangements.len() 
            - arrangements.iter().sum::<u8>() as usize;
            

        println!("Initial slack: {}", slack);

        let depth = " ";
        let last = recurse(records, arrangements, slack, depth);
        println!("Combos: {}", last);

        
    }

    println!("{sum}");
}

fn recurse(records: &[u8], arrangements: &[u8], slack: usize, depth: &str) -> u8 {
    if arrangements.len() == 0 {
        if records.iter().any(|byte| *byte == b'#') {
            println!("{depth}Forgot something!");
            return 0;
        }
        println!("{depth}Nothing left here! Looks good");
        return 1;
    }
    println!("{depth}Recurse with slack: {}, records: {:?}, arrangements: {:?}", slack, std::str::from_utf8(records).unwrap(), arrangements);
    let mut mandatory = false;
    let mut total = 0;
    let mut offset = 0;
    //go through each combination of spacing
    //
    while offset < min(slack + 1, records.len()) {
    // for mut offset in 0..(slack + 1) {
        println!("{depth}Loop with offset: {}", offset);
        for group_size in arrangements {
            let records = &records[offset..];

            // if *group_size as usize >= records.len() {
            //     println!("{depth}reached end too early");
            //     break;
            // } 

            if 
                (*group_size as usize) < records.len() &&
                records[
                    *group_size as usize
                ] == b'#' 
            {
                println!("{depth}smooshed into {}", records[*group_size as usize] as char);
                break;
            }

            if records[
                    ..
                    *group_size as usize
                ].into_iter().all(|byte| *byte != b'.') 
            {
                println!("{depth}Perfect fit at {:?}", std::str::from_utf8(&records[
                    ..
                    *group_size as usize
                ]).unwrap());
                if records[
                    ..
                    *group_size as usize
                ].into_iter().any(|byte| *byte == b'#') {
                    println!("{depth}Hit a broken pipe. Only valid spot");
                    mandatory = true;
                }
                total += recurse(
                    &records[min(1 + *group_size as usize, records.len())..], 
                    &arrangements[1..], 
                    slack - offset,
                    format!(" {depth}").as_str()
                );
                offset += 0;
                break;
            }

            println!("{depth}Ran into dot");

        }
        offset += 1;

        if mandatory {
            break;
        }
    }
    println!("{depth}Went through all {} offsets", slack + 1);
    return total;

}


// for group_size in arrangements {
        //     let mut offset = 0;
        //     while offset < min(slack + 1, records.len()) {
        //         //slack should maybe decrease
        //         let records = &records[offset..];

        //     if *group_size as usize >= records.len() {
        //         println!("reached end too early");
        //         break;
        //     } 

        //     if records[
        //         *group_size as usize
        //     ] == b'#' {
        //         println!("smooshed");
        //         continue;
        //     }

        //     if records[
        //         ..
        //         *group_size as usize
        //     ].into_iter().all(|byte| *byte != b'.') {
        //         println!("Perfect fit");
        //     }

        //     println!("Ran into dot");
        //     }
        // }

        // println!("{sum}");
        

        
            // if offset > slack {
            //     //yucky check to deal with the +3 skipahead
            //     break;
            // }

            // //+1 to search ahead for intersection
            // let stretch_size = min(*group_size as usize + offset + 1, records.len());
            // let perfect_fit = records[
            //     offset..
            //     (offset + *group_size as usize)
            // ].into_iter().all(|byte| *byte != b'.');

            // let spillage = records[
            //     (offset + *group_size as usize)..
            //     stretch_size
            // ]
            // .into_iter()
            // .any(|byte| *byte == b'#');
            // println!("{depth}perfect fit at {:?}: {}. spillage at {:?}: {}", offset..
            // (offset + *group_size as usize), perfect_fit, (offset + *group_size as usize)..
            // stretch_size, spillage);
            // if 
            //     //if not perfect match
            //     !perfect_fit
            //     ||
            //     //or spills over
            //     spillage
            // {
            //     println!("{depth}No! It Failed! Breaking!");
            //     // offset += 3;
            //     offset += 1;
            //     break;
            // } else {
            //     println!("{depth}Yes!");
            //     let next_is_issue = 0;

            //     println!("{depth}{}, {}", slack, offset);
            //     total += recurse(
            //             &records[next_is_issue + stretch_size..], 
            //             &arrangements[1..], 
            //             slack - offset
            //         );
            // }
            
            // offset += 1;