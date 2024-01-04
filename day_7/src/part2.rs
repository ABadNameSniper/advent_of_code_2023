use std::{fs, collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt").unwrap();
    let mut ranks = vec![HashMap::<u32, Vec::<u32>>::new(); 7];

    input
    .lines()
    .for_each(|line| {
        let mut m: HashMap<u32, _> = HashMap::<u32, u8>::new();
        let power: u32 = line[0..5]
        .chars()
        .rev()
        .enumerate()
        .map(|(index, char)| {
            //each value is from 0 to 12. That's 4 bits or less
            val_conversion(char) << (4*index)//+ (index as u32)//index as u32 * 13
        })
        .sum();

        line[0..5]
        .chars()
        .for_each(|char|{
            *m.entry(val_conversion(char))
            .or_default() += 1;
        });

        //separate jokers from the rest -- remember their value is 0
        //is this a magic numbers moment?
        let jokers = **(&m.get(&0).unwrap_or(&0));
        m.entry(0).and_modify(|v| *v = 0);

        let mut amounts: Vec<u8> = m.into_values().collect();
        amounts.sort_unstable();
        amounts.reverse();
        let max = *amounts.get(0).unwrap_or(&0) + jokers;
        let secondary = *amounts.get(1).unwrap_or(&0);
        
        let hand_type = match max {
            5 => 6,
            4 => 5,
            3 => 3,
            2 => 1,
            _ => 0
        } + (secondary > 1) as u32;


        let bid = line[6..line.len()].parse::<u32>().unwrap();

        // println!("Hand: {}, Type: {hand_type}, Pow: {power}, Jokers: {jokers}, Max: {max}, Sec: {secondary}, Bid: {bid}", &line[0..5]);
        ranks[hand_type as usize].entry(power).or_default().push(bid);

    });

    let mut winnings = 0;
    let mut rank = 1;
    for hand_type in ranks.iter() {
        let mut keys = hand_type.keys().collect::<Vec<_>>();
        // println!("{:?}", keys);
        keys.sort_unstable();
        keys.iter().for_each(|key| {
            for bid in &hand_type[*key] {
                winnings += rank * bid;
                rank += 1;
            }
        });
    }

    println!("{}, {}", winnings, now.elapsed().as_micros());

}

fn val_conversion(c: char) -> u32 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        'J' => 0,//Jokers are zero now!
        num => {
            // '2' -> 1
            num as u32 - 49
        },
    }
}