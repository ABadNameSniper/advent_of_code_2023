use std::time::Instant;
fn main() {
    let now = Instant::now();
    let og_time: u64 = 48_989_083;
    let distance: u64 = 390_110_311_121_360;

    let mut hold = 0;
    let mut time = og_time;
    while hold * time <= distance {
        hold += 1;
        time -= 1;
    }

    println!("{}, {}", time - hold + 1, now.elapsed().as_nanos());

}
