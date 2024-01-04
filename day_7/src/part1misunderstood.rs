use std::{fs, collections::HashMap};
//i didn't read hte instructions and thought a card would win based upon if the card it had duplicates of was higher than the others...
//also I did not read the section about the hand types... and i don't player poker
fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let mut ranks = vec![HashMap::<u8, Vec::<u32>>::new(); 7];

    input
    .lines()
    .for_each(|line| {
        //perfect: https://users.rust-lang.org/t/frequency-of-an-element-in-the-vector/43103/6
        let mut m: HashMap<u8, _> = HashMap::<u8, u8>::new();
        line[0..5]
        .chars()
        .for_each(|char|{
            *m.entry(match char {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                num => {
                    // '2' -> 0
                    num as u8 - 50
                },
            })
            .or_default() += 1;
        });
        let bid = line[6..line.len()].parse::<u32>().unwrap();
        let max = m.iter().max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap();
        let hand_type = m[max];

        ranks[hand_type as usize].entry(*max).or_default().push(bid);

    });

    // println!("{:?}", &input);
    println!("{:?}", ranks);

    let mut winnings = 0;
    let mut rank = 1;
    for hand_type in ranks.iter().rev() {
        let mut keys = hand_type.keys().collect::<Vec<_>>();
        println!("{:?}", keys);
        keys.sort_unstable();
        keys.iter().rev().for_each(|key| {
            for bid in &hand_type[*key] {
                winnings += rank * bid;
                rank += 1;
            }
        });
    }

    println!("{}, {}", winnings, rank);
}
