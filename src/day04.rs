use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day04.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 04!");
    let mut contain_cnt = 0;
    let mut overlap_cnt = 0;
    for line in lines {
        if let Ok(line) = line {
            if line.len() < 6 {
                continue;
            }
            let mut range = line.split(',').map(|range| range.split('-').map(|x| x.trim().parse::<i32>().unwrap())).flatten();
            let (l1, r1, l2, r2) = (range.next().unwrap(), range.next().unwrap(), range.next().unwrap(), range.next().unwrap());
            if (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2) {
                contain_cnt += 1;
            }
            if l1 <= r2 && l2 <= r1 {
                overlap_cnt += 1;
            }
        }
    }
    println!("contains => {}\noverlaps => {}", contain_cnt, overlap_cnt);
}
