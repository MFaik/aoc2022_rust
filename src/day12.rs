use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
use crate::vector2::Vector2;

type Tile = Vector2<usize>;

pub fn solve() {
    let file = File::open("./inputs/day12.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines().map(|x| x.unwrap());
    let map_width = lines.next().unwrap().len();
    let map_height = lines.fold(1, |acc, _| acc + 1);

    let file = File::open("./inputs/day12.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines().map(|x| x.unwrap());
    println!("day 12!");
    let mut height_map = vec![vec![0; map_width]; map_height]; 
    let mut distance_from_start = vec![vec![None; map_width]; map_height];
    let mut distance_from_end = vec![vec![None; map_width]; map_height];
    //normally I'd make this a priorty heap 
    //but for an input as like this the default queue works better
    let mut dfs_queue = VecDeque::new();
    let mut end = (0, 0);
    for (y, line) in lines.enumerate() {
        for (x, chr) in line.bytes().enumerate() {
            if chr == b'S' {
                height_map[y][x] = b'a' as i32;
                dfs_queue.push_front(Tile{x, y});
                distance_from_start[y][x] = Some(0);
            } else if chr == b'E' {
                height_map[y][x] = b'z' as i32;
                end = (x, y);
                distance_from_end[y][x] = Some(0);
            } else {
                height_map[y][x] = chr as i32;
            }
        }
    }
    while !dfs_queue.is_empty() {
        let current_tile = dfs_queue.pop_back().unwrap();
        let new_distance = distance_from_start[current_tile.y][current_tile.x].unwrap() + 1;
        for delta in [(0, 1), (0, -1), 
                        (1, 0), (-1, 0)] {
            //there has to be a better way to do this 
            if (current_tile.x == 0 && delta.0 == -1) 
                || (current_tile.y == 0 && delta.1 == -1){
                continue;    
            }
            let new_tile = Tile{x: (current_tile.x as i32 + delta.0) as usize, 
                                    y: (current_tile.y as i32 + delta.1) as usize};
            //usize stuff is really tripping me up

            if new_tile.x >= map_width || new_tile.y >= map_height {
                continue;
            }
            if height_map[new_tile.y][new_tile.x] - height_map[current_tile.y][current_tile.x] > 1 {
                continue;
            }
            if let Some(distance) = distance_from_start[new_tile.y][new_tile.x] {
                if new_distance >= distance {
                    continue;
                }
            }

            distance_from_start[new_tile.y][new_tile.x] = Some(new_distance);
            dfs_queue.push_front(new_tile);
        }
    }
    
    if let Some(distance) = distance_from_start[end.1][end.0] {
        println!("Mininum distance from start => {}", distance);
    } else {
        println!("Can't reach the end from the starting point");
    }

    dfs_queue.push_front(Tile{x: end.0, y: end.1});
    let mut closest_lowest_point_distance = None;
    while !dfs_queue.is_empty() {
        let current_tile = dfs_queue.pop_back().unwrap();
        let new_distance = distance_from_end[current_tile.y][current_tile.x].unwrap() + 1;
        for delta in [(0, 1), (0, -1), 
                        (1, 0), (-1, 0)] {
            //there has to be a better way to do this 
            if (current_tile.x == 0 && delta.0 == -1) 
                || (current_tile.y == 0 && delta.1 == -1){
                continue;    
            }
            let new_tile = Tile{x: (current_tile.x as i32 + delta.0) as usize, 
                                    y: (current_tile.y as i32 + delta.1) as usize};
            //usize stuff is really tripping me up

            if new_tile.x >= map_width || new_tile.y >= map_height {
                continue;
            }
            if height_map[new_tile.y][new_tile.x] - height_map[current_tile.y][current_tile.x] < -1 {
                continue;
            }
            if let Some(distance) = distance_from_end[new_tile.y][new_tile.x] {
                if new_distance >= distance {
                    continue;
                }
            }

            distance_from_end[new_tile.y][new_tile.x] = Some(new_distance);
            dfs_queue.push_front(new_tile);
        }
        if height_map[current_tile.y][current_tile.x] == b'a' as i32 {
            if let Some(low_point_dis) = closest_lowest_point_distance {
                if low_point_dis <= distance_from_end[current_tile.y][current_tile.x].unwrap() {
                    continue;
                }
            }
            closest_lowest_point_distance = distance_from_end[current_tile.y][current_tile.x];
        }
    }
    if let Some(distance) = closest_lowest_point_distance {
        println!("Distance to the best place to slide down to is => {}", distance);
    } else {
        println!("Can't reach the ground from the end");
    }
}
