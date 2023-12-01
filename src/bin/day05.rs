use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;

fn main() {
    part1();
    part2();
}


fn part1() {
    let file_string = read_to_string("inputs/day05.txt")
        .expect("File to be available");
    let mut lines = file_string.lines().into_iter();

    let stacks = parse_initial_state(&mut lines);

    while let Some(line) = lines.next() {
        let (from_stack_id, to_stack_id, amount) = parse_movement(line);

        let mut from_stack = stacks.get(&from_stack_id)
            .expect("From stack is available")
            .borrow_mut();
        let mut to_stack = stacks.get(&to_stack_id)
            .expect("To stack is available")
            .borrow_mut();

        for _ in 0..amount {
            let c = from_stack.pop().expect("Elements available in the stack");
            to_stack.push(c);
        }
    }

    let result = top_of_each_stack(stacks);
    println!("Part1: top of each stack {}", result);
}

fn part2() {
    let file_string = read_to_string("inputs/day05.txt")
        .expect("File to be available");
    let mut lines = file_string.lines().into_iter();

    let stacks = parse_initial_state(&mut lines);

    while let Some(line) = lines.next() {
        let (from_stack_id, to_stack_id, amount) = parse_movement(line);

        let mut from_stack = stacks.get(&from_stack_id)
            .expect("From stack is available")
            .borrow_mut();
        let mut to_stack = stacks.get(&to_stack_id)
            .expect("To stack is available")
            .borrow_mut();

        let range = (from_stack.len() - (amount as usize))..;
        let mut crates = from_stack.drain(range).collect::<Vec<char>>();
        to_stack.append(&mut crates);
    }


    let result = top_of_each_stack(stacks);

    println!("Part2: top of each stack {}", result);
}

fn parse_initial_state(lines: &mut Lines) -> HashMap<i32, RefCell<Vec<char>>> {
    let mut stacks = HashMap::from([
        (1, RefCell::new(Vec::new())),
        (2, RefCell::new(Vec::new())),
        (3, RefCell::new(Vec::new())),
        (4, RefCell::new(Vec::new())),
        (5, RefCell::new(Vec::new())),
        (6, RefCell::new(Vec::new())),
        (7, RefCell::new(Vec::new())),
        (8, RefCell::new(Vec::new())),
        (9, RefCell::new(Vec::new())),
    ]);

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        } else {
            let mut stack_id = 0;
            for (idx, c) in line.chars().enumerate() {
                if idx % 4 == 0 {
                    stack_id += 1
                }

                if ('A'..='Z').contains(&c) {
                    let stack = stacks.get_mut(&stack_id)
                        .expect("Stack is present");
                    stack.get_mut().insert(0, c);
                }
            }
        }
    }

    stacks
}

fn parse_movement(line: &str) -> (i32, i32, i32) {
    let [_, amount, _, from, _, to, ..] = line.splitn(6, " ").collect::<Vec<&str>>()[..]
        else { panic!("Incorrect format while splitting movement") };

    (
        from.parse::<i32>().expect("From stack is an int"),
        to.parse::<i32>().expect("To stack is an int"),
        amount.parse::<i32>().expect("Amount is an int"),
    )
}

fn top_of_each_stack(stacks: HashMap<i32, RefCell<Vec<char>>>) -> String {
    let mut entries = stacks.iter()
        .collect::<Vec<(&i32, &RefCell<Vec<char>>)>>();

    entries.sort_by(|(stack_id_1, _), (stack_id_2, _)| stack_id_1.cmp(&stack_id_2));

    let result = entries.iter()
        .fold(String::from(""), |acc, (_, stack)|
            match stack.borrow().last() {
                Some(c) => acc + c.to_string().as_str(),
                None => acc,
            });
    result
}

