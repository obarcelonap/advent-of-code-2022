use std::cmp::max;
use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in read_to_string("inputs/day01.txt").unwrap().lines() {
        if line.trim().is_empty() {
            max_calories = max(max_calories, current_calories);
            current_calories = 0;
        } else {
            current_calories += line.trim().parse::<i32>()
                .expect("Expect line to be parsed as int");
        }
    }
    println!("Part1: max calories are {}", max_calories);
}

fn part2() {
    let mut max_calories = Vec::with_capacity(3);
    let mut current_calories = 0;
    for line in read_to_string("inputs/day01.txt").unwrap().lines() {
        if line.trim().is_empty() {
            max_calories.push(current_calories);
            max_calories.sort();
            max_calories.reverse();
            if max_calories.len() > 3 {
                max_calories.truncate(3);
            }
            current_calories = 0;
        } else {
            current_calories += line.trim().parse::<i32>()
                .expect("Expect line to be parsed as int");
        }
    }

    let top3_total_calories = max_calories.iter().copied().reduce(|acc, e| acc + e).unwrap();
    println!("Part2: top3 total calories are {}", top3_total_calories);
}
