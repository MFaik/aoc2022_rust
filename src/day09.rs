use std::fs::File;
use std::io::BufRead;
use crate::vector2::Vector2;
use std::collections::HashSet;

const ROPE_LEN : usize = 10;

pub fn solve() {
    let file = File::open("./inputs/day09.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 09!");
    let mut rope = [Vector2{x: 0, y: 0}; ROPE_LEN]; 
    let mut visited_places_part1 = HashSet::new();
    let mut visited_places_part2 = HashSet::new();
    for line in lines {
        if let Ok(line) = line {
            let mut line = line.split(' ');
            let direction = line.next().unwrap().chars().nth(0).unwrap();
            for _ in 0..line.next().unwrap().parse().unwrap() {
                //move the head
                match direction {
                    'R' => rope[0] += Vector2{x: 1, y: 0},
                    'L' => rope[0] += Vector2{x: -1,y: 0},
                    'U' => rope[0] += Vector2{x: 0, y: 1},
                    'D' => rope[0] += Vector2{x: 0, y:-1},
                    _ => panic!()
                };
                for i in 1..ROPE_LEN {
                    let head = rope[i-1];
                    let tail = &mut rope[i];
                    //does tail need to move?
                    let mut move_tail = false;
                    if (head.x - tail.x).abs() > 1 {
                        move_tail = true;
                    } else if (head.y - tail.y).abs() > 1 {
                        move_tail = true;
                    }
                    //move the tail
                    if move_tail {
                        if head.x < tail.x { 
                            tail.x -= 1;
                        } else if head.x > tail.x {
                            tail.x += 1;
                        }
                        if head.y < tail.y {
                            tail.y -= 1;
                        } else if head.y > tail.y {
                            tail.y += 1;
                        }
                    }
                }

                visited_places_part1.insert((rope[1].x, rope[1].y));
                visited_places_part2.insert((rope[9].x, rope[9].y));
            }

        }
    }
    println!("(Part1)The amount of places visited by tail: {}", visited_places_part1.len());
    println!("(Part2)The amount of places visited by tail: {}", visited_places_part2.len());
}
