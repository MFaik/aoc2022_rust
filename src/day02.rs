use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day02.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 02!");
    let mut score = 0;
    for line in lines {
        if let Ok(line) = line {
            if line.len() < 2 {
                continue;
            }
            let opponent = match line.chars().nth(0) {
                Some('A') => 1,
                Some('B') => 2,
                Some('C') => 3,
                _ => panic!()
            };
            //part 1
            //let me = match line.chars().nth(2) {
            //    Some('X') => 1,
            //    Some('Y') => 2,
            //    Some('Z') => 3,
            //    _ => panic!()
            //};

            //part 2
            let me = match line.chars().nth(2) {
                Some('X') => (opponent-2+3)%3+1,
                Some('Y') => opponent,
                Some('Z') => (opponent)%3+1,
                _ => panic!()
            };
            score += me;
            if me == opponent {
                score += 3;
            } else if me - opponent == 1 || me - opponent == -2 {
                score += 6;
            }
        }
    }
    println!("rps score => {}", score);
}
