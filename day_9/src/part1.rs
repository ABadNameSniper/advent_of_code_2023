use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut histories: Vec<Vec<Vec<i32>>> = input
    .lines()
    .map(|line|
        vec![
            line.split(" ")
            .map(|numstir|
                numstir.parse().unwrap()
            )
            .collect()
        ]
    )
    .collect();

    println!("{:?}", histories);

    //I had an idea to use the absolute minimum amount of RAM as required
    //but kind of gave up on that once I realized i could do it differently
    //and now there's spaghetti
    let mut sum = 0;
    for history in histories.iter_mut() {
        let mut seq = history
        .first()
        .unwrap()
        .clone();

        let mut seq_vec = vec![seq.clone()];//Vec::new();//vec![seq.clone()];
        while !seq.iter().all(|num| *num == 0) {
            seq = (0..seq.len()-1)
            .map(|index| {
                seq[index + 1] - seq[index]
            })
            .collect();
            seq_vec.push(seq.clone());
        }

        drop(seq);

        let add_zero_seq = seq_vec.len() -1 ;
        seq_vec[add_zero_seq].push(0);
        
        (1..seq_vec.len())
        .rev()
        .for_each(|index| {
            let old_seq_vec = seq_vec.clone();
            seq_vec[index - 1].push(
                old_seq_vec[index].last().unwrap() +
                old_seq_vec[index - 1].last().unwrap()
            );
            println!("Pushed to: {}. See? {:?} also {}", index - 1, seq_vec, old_seq_vec[index].last().unwrap() +
            old_seq_vec[index - 1].last().unwrap());
        });



        println!("{:?}", seq_vec);


        // seq_vec.iter().for_each(|seq| history.push(seq.clone()));
        sum += seq_vec.first().unwrap().last().unwrap();
    }
    println!("{sum}");
    // println!("{:?}", histories);
    // let sum = histories.iter().map(|history| history.first().unwrap().last().unwrap()).sum::<i32>();
    // println!("{sum}");
}
