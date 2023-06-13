use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day10.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 10!");
    let mut x : i32 = 1;
    let mut cycle = 0;
    let mut target = 20;
    let mut signal_sum = 0;
    let mut screen = vec!['.'; 240];
    screen[0] = '#';
    for line in lines {
        if let Ok(line) = line {
            let mut line = line.split(' ');
            let op = line.next().unwrap();
            
            cycle += 1;
            if cycle == target {
                signal_sum += x*cycle;
                target += 40;
            }
            if (cycle%40 - x).abs() <= 1 {
                screen[cycle as usize] = '#';
            }
            match op {
                "addx" => {
                    cycle += 1;
                    if cycle == target {
                        signal_sum += x*cycle;
                        target += 40;
                    }
                    x += line.next().unwrap().parse::<i32>().unwrap();
                    if (cycle%40 - x).abs() <= 1 {
                        screen[cycle as usize] = '#';
                    }
                },
                _ => ()
            }
        }
    }
    println!("Signal Sum => {}", signal_sum);
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", screen[x+y*40]);
        }
        println!("");
    }
}
