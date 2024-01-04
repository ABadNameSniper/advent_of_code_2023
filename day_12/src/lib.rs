use std::cmp::min;
use rayon::prelude::*;


pub fn recurse(records: &[u8], arrangements: &[u8]) -> usize {
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
    let mut total: usize = 0;


    let slack =  records.len()
        + 2
        - arrangements.len() 
        - arrangements.iter().sum::<u8>() as usize;

    for offset in 0..slack {
    // (0..slack).par_iter().for_each(|offset| {
        let records = &records[offset..];

        let place_here_now = match records.first() {
            Some(val) => *val == b'#',
            _ => false
        };

        if //check if the one right after group would mesh together
            (group_size) < records.len() &&
            records[
                group_size
            ] == b'#' 
        {
            //println!("{depth}smooshed into {}", records[group_size] as char);
            if place_here_now { break }
            continue;
        }

        let potential_group = &records[0..min(group_size, records.len())];
        //if no dots, it's a match
        if !potential_group.iter().any(|byte| *byte == b'.') {
            //println!("{depth}the shoe {} fits in {:?}", group_size, &potential_group);
            total += recurse(
                &records[min(1 + group_size /*+ breathing_room*/, records.len())..],
                &arrangements[1..], 
            );
        }
        if place_here_now { break }

    }//)
    
    //println!("{depth}Went through all \"{}\" offsets", slack + 1);

    total

}