use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day06.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 06!");
    //part 1
    //let mark_len = 4;
    //part 2
    let mark_len = 14;
    for line in lines {
        if let Ok(line) = line {
            let line : Vec<u8> = line.bytes().collect();
            'outer: for range in 0..=line.len()-mark_len {
                for i in range..range+mark_len-1 {
                    for j in i+1..range+mark_len {
                        if line[i] == line[j] {
                            continue 'outer;
                        }
                    }
                }
                println!("mark found after {} letters", range+mark_len);
                return;
            }
        }
    }
}
