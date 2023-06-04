use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

pub struct Graph {
    nodes: Vec<NodeData>
}

pub struct NodeData {
    size: i64,
    parent: Option<NodeIndex>,
    children: HashMap<String, NodeIndex>
}
pub type NodeIndex = usize;

impl Graph {
    pub fn add_node(&mut self, size: i64, name: String, parent:Option<NodeIndex>) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData{size, parent, children: HashMap::new()});
        if let Some(parent) = parent {
            self.nodes[parent].children.insert(name, index);
        }
        index
    }
}

const DISK_SPACE : i64 = 70_000_000;
const SPACE_NEED : i64 = 30_000_000;

impl Graph {
    pub fn calculate_small_size(&self, current_node: NodeIndex, mut small_dir_total: &mut i64) -> i64 {
        let mut total_size = self.nodes[current_node].size;
        if total_size == 0 {
            for (_, index) in &self.nodes[current_node].children {
                total_size += self.calculate_small_size(*index, &mut small_dir_total);     
            }
            if total_size < 100000 {
                *small_dir_total += total_size;
            }
        }
        total_size
    }

    pub fn find_small_directory(&self, current_node: NodeIndex, min_size: i64, mut dir_size: &mut i64) -> i64 {
        let mut total_size = self.nodes[current_node].size;
        if total_size == 0 {
            for (_, index) in &self.nodes[current_node].children {
                total_size += self.find_small_directory(*index, min_size, &mut dir_size);     
            }
            if total_size > min_size && total_size < *dir_size {
                *dir_size = total_size;
            }
        }
        total_size
    }
}

pub fn solve() {
    let file = File::open("./inputs/day07.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    println!("day 07!");
    let mut file_system = Graph{nodes: Vec::new()};
    let mut current_file = file_system.add_node(0, "/".to_string(), None);
    for line in lines {
        if let Ok(line) = line {
            let words : Vec<&str> = line.split(' ').collect();
            match words[0] {
                "$" => {
                    if words[1] == "ls"{
                        continue;
                    } else {
                        if words[2] == ".." {
                            current_file = file_system.nodes[current_file].parent.unwrap();
                        } else if words[2] == "/" {
                            current_file = 0;
                        } else {
                            current_file = file_system.nodes[current_file].children[words[2].trim()];
                        }
                    }
                },
                "dir" => {
                    file_system.add_node(0, words[1].to_string(), Some(current_file));
                },
                size => {
                    let size = size.parse::<i64>().unwrap(); 
                    file_system.add_node(size, words[1].to_string(), Some(current_file));
                }
            };
        }
    }
    let mut small_dir_total = 0;
    let total_size = file_system.calculate_small_size(0, &mut small_dir_total);
    println!("total size => {}", total_size);
    println!("small directory size => {}", small_dir_total);
    
    let mut dir_size = DISK_SPACE;
    file_system.find_small_directory(0, SPACE_NEED - (DISK_SPACE-total_size), &mut dir_size);
    println!("deleted file size => {}", dir_size);
}
