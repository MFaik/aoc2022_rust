use std::fs::File;
use std::io::BufRead;

pub fn solve() {
    let file = File::open("./inputs/day08.txt").unwrap();
    let buf = std::io::BufReader::new(file);
    let lines = buf.lines();
    println!("day 08!");
    let mut grid = Vec::new();
    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            grid.push(Vec::new());
            for c in line.bytes() {
                grid[i].push((c-b'0') as i8);
            }
        }
    }
    //part 1
    let mut is_tree_visible = vec![vec![false; grid[0].len()]; grid.len()];
    for y in 0..grid.len() {
        let mut last_visible = -1;
        for x in 0..grid[y].len() {
            if grid[y][x] > last_visible {
                is_tree_visible[y][x] = true;
                last_visible = grid[y][x];
            }
        }

        let mut last_visible = -1;
        for x in (0..grid[y].len()).rev() {
            if grid[y][x] > last_visible {
                is_tree_visible[y][x] = true;
                last_visible = grid[y][x];
            }
        }
    }
    for x in 0..grid[0].len() {
        let mut last_visible = -1;
        for y in 0..grid.len() {
            if grid[y][x] > last_visible {
                is_tree_visible[y][x] = true;
                last_visible = grid[y][x];
            }
        }
        let mut last_visible = -1;
        for y in (0..grid.len()).rev() {
            if grid[y][x] > last_visible {
                is_tree_visible[y][x] = true;
                last_visible = grid[y][x];
            }
        }
    }
    let mut cnt = 0;
    for line in is_tree_visible {
        for vis in line {
            if vis {
                cnt += 1;
            }
        }
    }
    println!("visible trees => {}", cnt);
    let mut max_view = 0;
    //part 2
    //I can probably do a segment tree for part2 but who cares the grid is too small
    for current_y in 0..grid.len() {
        for current_x in 0..grid[0].len() {
            let mut current_scene = 1;

            let mut current_view = 0;
            for x in current_x+1..grid[0].len() {
                current_view += 1;
                if grid[current_y][x] >= grid[current_y][current_x] {
                    break;
                }
            }
            current_scene *= current_view;

            let mut current_view = 0;
            for x in (0..current_x).rev() {
                current_view += 1;
                if grid[current_y][x] >= grid[current_y][current_x] {
                    break;
                }
            }
            current_scene *= current_view;

            let mut current_view = 0;
            for y in current_y+1..grid.len() {
                current_view += 1;
                if grid[y][current_x] >= grid[current_y][current_x] {
                    break;
                }
            }
            current_scene *= current_view;

            let mut current_view = 0;
            for y in (0..current_y).rev() {
                current_view += 1;
                if grid[y][current_x] >= grid[current_y][current_x] {
                    break;
                }
            }
            current_scene *= current_view;

            if max_view < current_scene {
                max_view = current_scene;
            }
        }
    }
    println!("max view => {}", max_view);
}
