use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
struct ArrayElement {
    number: Option<i32>,
    array: Vec<ArrayElement>
}

impl PartialOrd for ArrayElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArrayElement {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.number != None && other.number != None {
            return self.number.unwrap().cmp(&other.number.unwrap());
        }
        if self.number != None && other.number == None {
            let self_deep = &ArrayElement{number: None, array: vec![ArrayElement{number: self.number, array: Vec::new()}]};
            return self_deep.cmp(other);
        }
        if self.number == None && other.number != None {
            let other_deep = &ArrayElement{number: None, array: vec![ArrayElement{number: other.number, array: Vec::new()}]};
            return self.cmp(other_deep);
        }
        let mut i = 0;
        loop {
            if i >= self.array.len() && i >= other.array.len() {
                return Ordering::Equal;
            }
            if i >= self.array.len() {
                return Ordering::Less;
            }
            if i >= other.array.len() {
                return Ordering::Greater;
            }
            match &self.array[i].cmp(&other.array[i]) {
                Ordering::Equal => {},
                comp => {
                    return *comp;
                }
            }
            i += 1;
        }
    }
}

fn read_until_closure(str: &[u8]) -> (ArrayElement, usize) {
    //we start at offset 1 to skip the first '[' char
    let mut i = 1;
    let mut element = ArrayElement{number: None, array: Vec::new()};
    while i < str.len() {
        match str[i] {
            b'[' => {
                let (child, skip) = read_until_closure(&str[i..]);
                element.array.push(child);
                i += skip;
            },
            b']' => {
                i += 1;
                break; 
            },
            b' ' | b',' => {},
            _ => {
                let mut number = 0;
                loop {
                    number *= 10;
                    number += (str[i]-b'0') as i32;
                    i += 1;
                    if i >= str.len() || !str[i].is_ascii_digit()  {
                        break;
                    }
                }
                i -= 1;
                let child = ArrayElement{number: Some(number), array: Vec::new()};
                element.array.push(child);
            }
        };
        i += 1;
    }
    (element, i)
}

pub fn solve() {
    let file = File::open("./inputs/day13.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines().map(|x| x.unwrap());
    println!("day 13!");
    let mut index = 1;
    let mut index_sum = 0;
    let mut package_vec = Vec::new();
    loop {
        let left_str = match lines.next() {
            Some(str) => str,
            None => break
        };
        let right_str = lines.next().unwrap();
        lines.next();
        let (left, _) = read_until_closure(&left_str.bytes().collect::<Vec<u8>>());
        let (right, _) = read_until_closure(&right_str.bytes().collect::<Vec<u8>>());

        if left < right {
            index_sum += index;
        }

        package_vec.push(left);
        package_vec.push(right);

        index += 1;
    }
    println!("Sum of the ordered indexes => {}", index_sum);
    
    package_vec.push(read_until_closure(&"[[2]]".bytes().collect::<Vec<u8>>()).0);
    package_vec.push(read_until_closure(&"[[6]]".bytes().collect::<Vec<u8>>()).0);

    package_vec.sort();
        
    let mut decoder_key = 1;
    for (i, element) in package_vec.iter().enumerate() {
        if element.array.len() == 1 {
            if element.array[0].array.len() == 1 {
                match element.array[0].array[0].number {
                    Some(2) | Some(6) => { decoder_key *= i+1; },
                    _ => {}
                }
            }
        } 
    }
    println!("Decoder key => {}", decoder_key);
}
