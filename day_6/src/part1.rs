fn main() {
    let time = vec![48, 98, 90, 83];
    let distance = vec![390, 1103, 1112, 1360];

    // let time = vec![7, 15, 30];
    // let distance = vec![9, 40, 200];

    println!("{}", time.iter().enumerate().map(|(index, og_time)| {
        let mut hold = 0;
        let mut time = *og_time;
        while hold * time <= distance[index] {
            hold += 1;
            time -= 1;
        }

        time - hold + 1

    }).product::<i32>())
}
