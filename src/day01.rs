use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day01.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 01!");
    let mut elfs = Vec::new();
    let mut current_sum = 0;
    for line in lines {
        if let Ok(ip) = line {
            if ip == "" {
                elfs.push(current_sum);
                current_sum = 0;
            } else {
                current_sum += ip.parse::<i64>().unwrap();
            }
        }
    }
    elfs.sort_unstable_by(|a, b| b.cmp(a));
    println!("highest calorie elf => {}", elfs[0]);
    println!("total of top 3 elfs => {}", elfs[0] + elfs[1] + elfs[2]);
}
