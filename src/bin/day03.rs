use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    day1();
    day2();
}

fn day1() {
    let priorities: HashMap<char, i32> = HashMap::from_iter(
        ('a'..='z').zip(1..=26)
            .chain(('A'..='Z').zip(27..=52))
    );

    let mut total_priorities = 0;

    for line in read_to_string("inputs/day03.txt").unwrap().lines() {
        let (rucksack1, rucksack2) = line.split_at(line.len() / 2);

        let equal_items = intersection(&rucksack1, &rucksack2);
        let item = equal_items
            .first()
            .expect("One equal item is found in both rucksacks");

        let item_priority = priorities.get(item)
            .expect("Equal item has a priority");

        total_priorities += item_priority;
    };

    println!("Part1: total priorities {}", total_priorities);
}

fn day2() {
    let priorities: HashMap<char, i32> = HashMap::from_iter(
        ('a'..='z').zip(1..=26)
            .chain(('A'..='Z').zip(27..=52))
    );

    let mut total_priorities = 0;

    let file_string = read_to_string("inputs/day03.txt")
        .expect("File to be available");
    let mut lines = file_string.lines().into_iter();

    while let Some(rucksack1) = lines.next() {
        let rucksack2 = lines.next().expect("2nd rucksack is available");
        let rucksack3 = lines.next().expect("3rd rucksack is available");


        let equal_items = intersection3(&rucksack1, &rucksack2, &rucksack3);

        let item = equal_items
            .first()
            .expect("One equal item is found in both rucksacks");

        let item_priority = priorities.get(item)
            .expect("Equal item has a priority");

        total_priorities += item_priority;
    }

    println!("Part2: total priorities {}", total_priorities);
}

fn intersection(s1: &str, s2: &str) -> Vec<char> {
    let mut found = Vec::new();
    for c in s1.chars() {
        if s2.contains(c) {
            found.push(c);
        }
    }
    found.dedup();
    found
}

fn intersection3(s1: &str, s2: &str, s3: &str) -> Vec<char> {
    let mut found = Vec::new();
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            found.push(c);
        }
    }
    found.dedup();
    found
}

