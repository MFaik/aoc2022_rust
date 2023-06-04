use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day03.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 03!");
    let mut priority : i64 = 0;
    //part 1
    //    for line in lines {
    //        if let Ok(line) = line {
    //            'outer: for left in line.bytes().take(line.len()/2) {
    //                for right in line.bytes().skip(line.len()/2) {
    //                    if left == right {
    //                        let mut left = left;
    //                        if left < 97 {
    //                            left += 58;
    //                        }
    //                        priority += (left-('a' as u8) + 1) as i64;
    //                        break 'outer;
    //                    }
    //                }
    //            }
    //        }
    //    }

    //part 2
    let lines : Vec<String> = lines.filter_map(|x| x.ok()).collect();
    for i in (0..lines.len()).step_by(3) {
        'outer: for a in lines[i].bytes() {
            for b in lines[i+1].bytes() {
                for c in lines[i+2].bytes() {
                    if a == b && b == c {
                        let mut a = a;
                        if a < 97 {
                            a += 58;
                        }
                        priority += (a-('a' as u8) + 1) as i64;
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("rucksack priority => {}", priority);
}
