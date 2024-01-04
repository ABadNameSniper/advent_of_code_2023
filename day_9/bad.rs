    // for mut history in histories {
    //     for (index, value) in history.iter_mut().enumerate().rev() {
    //         let sequence = value.iter().rev();
    //         let mut prev = value.last().unwrap();
    //         let new_seq: Vec<_> = value.iter().rev().map(|item| {
    //             let lower_val = prev - item;
    //             prev = item;
    //             lower_val
    //         }).collect();
    //         history.push(vec![1]);
    //     }
    // }

    // for history in histories.iter_mut() {
    //     let mut new_sequences: Vec<Vec<_>> = Vec::<Vec<i32>>::new();
        
    //     for value in history.iter_mut().rev() {
    //         println!(
    //             "{:?}", value
    //         );
    //         let mut prev = value.first().unwrap();
    //         let new_seq: Vec<_> = value.iter().map(|item| {
    //             let lower_val = item - prev;
    //             prev = item;
    //             lower_val
    //         }).collect();
    //         println!("{:?}", new_seq);
    //         new_sequences.push(new_seq);
    //     }
    //     loop {
            
    //     }

    //     history.extend(new_sequences);
    // }