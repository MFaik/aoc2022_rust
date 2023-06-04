use std::collections::LinkedList;
use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day05.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 05!");
    let mut stacks = vec!(LinkedList::<u8>::new() ; 10);
    for line in lines {
        if let Ok(line) = line {
            if line.chars().nth(0).unwrap() != 'm' {
                for (i, c) in line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate() {
                    if c.is_ascii_digit() {
                        continue;
                    } else if c != ' ' {
                        stacks[i].push_back(c as u8);
                    }
                }
                continue;
            }
            let mut step = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap());
            if let (Some(cnt), Some(from), Some(to)) = (step.next(), step.next(), step.next()) {
                //part 1
                //for _ in 0..cnt {
                //    let top = stacks[from-1].pop_front().unwrap();
                //    stacks[to-1].push_front(top);
                //}
                
                //part 2
                for _ in 0..cnt {
                    let top = stacks[from-1].pop_front().unwrap();
                    stacks[9].push_front(top);
                }
                for _ in 0..cnt {
                    let top = stacks[9].pop_front().unwrap();
                    stacks[to-1].push_front(top);
                }
            }
        }
    }
    print!("text on top of the stacks => ");
    for stack in stacks {
        print!("{}",*stack.front().unwrap_or(&b' ') as char );
    }
    println!("");
}
