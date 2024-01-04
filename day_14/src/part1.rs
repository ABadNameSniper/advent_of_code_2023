use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    //it's much easier to do it on its side
    let mut rotation = rotate(input);

    rotation.iter_mut().for_each(|line| {
        let mut last_open = 0;
        for current_index in 0..line.len() {
            let current_byte = line.get_mut(current_index).unwrap();
            let byte_val = *current_byte;
            if byte_val == b'#' {
                last_open = current_index + 1;
                continue;
            }
            if byte_val == b'.' {
                continue;
            }
            //must be O at this point
            
            *current_byte = b'.';
            line[last_open] = byte_val;
            
            last_open += 1;
            
        }
        // slid.push(line.clone());
    });

    //it's really more like a flip, you know?
    rotation = rotate(rotation);

    let mut sum = 0;
    for (index, line) in rotation.iter().enumerate() {
        for byte in line {
            if *byte == b'O' {
                sum +=  line.len() - index
            }
        }
    }

    println!("{sum}");
    // fs::write("output1.txt", rotation.iter().map(|line| std::str::from_utf8(&line).unwrap()).collect::<Vec<_>>().join("\n"));

}

fn rotate(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotation: Vec<Vec<u8>> = vec![Vec::new(); grid[0].len()];
    for line in grid {
        for (byte_index, byte) in line.as_slice().iter().enumerate() {
            rotation[byte_index].push(*byte);
        }
    }
    rotation
}