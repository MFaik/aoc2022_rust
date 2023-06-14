use std::fs::File;
use std::io::BufRead;


const ROUND_CNT_PART1 : i64 = 20;
const ROUND_CNT_PART2 : i64 = 10000;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Vec<i64>>,
    //items variable hold vectors that have modded values of the items in part2
    //because the values get too big, every item is modded for every monkey
    //but in part1 they kinda look weird but its kinda alright I guess?
    multiply_operator: bool,
    operation_number: i64, 
    //if operation is multiplication and the operation number is 0 I just treat it like its a square root operation
    //I know its hacky but adding more stuff just for a single monkey felt like overkill
    div_test: i64,
    true_throw: usize,
    false_throw: usize 
}

fn parse_monkeys(monkeys: &mut Vec<Monkey>, lines: Vec<String>) {
    for i in 0..lines.len()/7 {
        /*
           this is an example of a monkey
           Monkey 0:
           Starting items: 79, 98
           Operation: new = old * old 
           Test: divisible by 23
           If true: throw to monkey 2
           If false: throw to monkey 3

*/
        let mut items = Vec::new();
        lines[i*7 + 1]
            .split(':').nth(1).unwrap()
            .split(',')
            .for_each(|x| items.push(vec![x.trim().parse().unwrap()]));

        let mut operation = lines[i*7 + 2].split("old ").nth(1).unwrap().split(' ');
        let multiply_operator = operation.next().unwrap() == "*";
        let operation_number = operation.next().unwrap().parse().unwrap_or(0);

        let div_test = lines[i*7 + 3].chars().filter(|x| x.is_digit(10)).collect::<String>()
            .parse().unwrap();
        let true_throw = lines[i*7 + 4].chars().filter(|x| x.is_digit(10)).collect::<String>()
            .parse().unwrap();
        let false_throw = lines[i*7 + 5].chars().filter(|x| x.is_digit(10)).collect::<String>()
            .parse().unwrap();
        monkeys.push(Monkey{
            items,
            multiply_operator,
            operation_number,
            div_test,
            true_throw,
            false_throw
        })
    }   
}

fn calculate_throws_part1(monkeys: &mut Vec<Monkey>, monkey_inspect_cnt: &mut Vec<usize>) {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();
        monkey_inspect_cnt[i] += monkey.items.len();
        for mut item in monkey.items {
            if monkey.multiply_operator {
                if monkey.operation_number == 0 {
                    item[0] *= item[0];
                } else {
                    item[0] *= monkey.operation_number;
                }
            } else {
                item[0] += monkey.operation_number;
            }
            item[0] /= 3;
            let target_monkey = if item[0] % monkey.div_test == 0 {
                monkey.true_throw
            } else {
                monkey.false_throw
            };
            monkeys[target_monkey].items.push(item);
        }
        monkeys[i].items.clear();
    }
}

fn make_monkey_mod(monkeys: &mut Vec<Monkey>) -> Vec<i64> {
    let mod_list : Vec<i64> = monkeys.iter().map(|x| x.div_test).collect();
    for monkey in monkeys {
        for item in monkey.items.iter_mut() {
            let num = item[0];
            let mut new_item = Vec::new();
            for m in mod_list.iter() {
                new_item.push(num%m);
            }
            *item = new_item;
        }
    }
    mod_list
}

fn calculate_throws_part2(monkeys: &mut Vec<Monkey>, monkey_inspect_cnt: &mut Vec<usize>, mod_list: &Vec<i64>) {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();
        monkey_inspect_cnt[i] += monkey.items.len();
        for mut item in monkey.items {
            for (j, mod_item) in item.iter_mut().enumerate() {
                if monkey.multiply_operator {
                    if monkey.operation_number == 0 {
                        *mod_item *= *mod_item;
                    } else {
                        *mod_item *= monkey.operation_number;
                    }
                } else {
                    *mod_item += monkey.operation_number;
                }
                *mod_item %= mod_list[j];
            }
            let target_monkey = if item[i] == 0 {
                monkey.true_throw
            } else {
                monkey.false_throw
            };
            monkeys[target_monkey].items.push(item);
        }
        monkeys[i].items.clear();
    }
}

pub fn solve() {
    let file = File::open("./inputs/day11.txt").unwrap();
    let lines : Vec<String> = std::io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    println!("day 11!");
    let mut monkeys = Vec::new();
    parse_monkeys(&mut monkeys, lines);
    let mut monkeys_part2 = monkeys.clone();
    let mut monkey_inspect_cnt = vec![0; monkeys.len()];

    for _ in 0..ROUND_CNT_PART1 {
        calculate_throws_part1(&mut monkeys, &mut monkey_inspect_cnt);
    }
    monkey_inspect_cnt.sort_unstable();
    println!("Monkey Business part1 => {}", monkey_inspect_cnt.iter()
           .rev()
           .take(2)
           .fold(1,|acc, e| acc * e));

    let mut monkey_inspect_cnt = vec![0; monkeys.len()];
    let mod_list = make_monkey_mod(&mut monkeys_part2);

    for _ in 0..ROUND_CNT_PART2 {
        calculate_throws_part2(&mut monkeys_part2, &mut monkey_inspect_cnt, &mod_list);
    }
    monkey_inspect_cnt.sort_unstable();
    println!("Monkey Business part2 => {}", monkey_inspect_cnt.iter()
           .rev()
           .take(2)
           .fold(1,|acc, e| acc * e));
}
